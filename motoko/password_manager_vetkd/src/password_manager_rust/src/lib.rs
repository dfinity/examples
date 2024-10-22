use candid::Principal;
use ic_cdk_macros::*;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

pub mod vault;
use vault::{
    AccessChange, AccessRights, EncryptedPassword, PasswordId, PasswordVersion, PrincipalName, Vault, VaultId, VaultIds, VaultStatus, VaultVersion
};

pub mod vetkd_api_types;
use vetkd_api_types::{
    CanisterId, VetKDCurve, VetKDEncryptedKeyReply, VetKDEncryptedKeyRequest, VetKDKeyId,
    VetKDPublicKeyReply, VetKDPublicKeyRequest,
};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const VETKD_SYSTEM_API_CANISTER_ID: &str = "s55qq-oqaaa-aaaaa-aaakq-cai";

// We use a canister's stable memory as storage. This simplifies the code and makes the appliation
// more robust because no (potentially failing) pre_upgrade/post_upgrade hooks are needed.
// Note that stable memory is less performant than heap memory, however.
// Currently, a single canister smart contract is limited to 96 GB of stable memory.
// For the current limits see https://internetcomputer.org/docs/current/developer-docs/production/resource-limits.
// To ensure that our canister does not exceed the limit, we put various restrictions (e.g., number of users) in place.
static MAX_USERS: u64 = 1_000;
static MAX_PASSWORDS_PER_USER: usize = 500;
static MAX_PASSWORD_CHARS: usize = 100;
static MAX_SHARES_PER_VAULT: usize = 50;
static MAX_ACCESS_VAULTS_PER_USER: usize = 10;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    /// Vault IDs are *global*, unique, and monotonically increasing.
    static NEXT_PASSWORD_ID: RefCell<StableCell<PasswordId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(0))),
            0
        ).expect("failed to init NEXT_PASSWORD_ID")
    );

    /// Maps a password-manager-global password ID to an encrypted password.
    static PASSWORDS: RefCell<StableBTreeMap<PasswordId, EncryptedPassword, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1))),
        )
    );

    /// Vault IDs are global, unique, and monotonically increasing.
    static NEXT_VAULT_ID: RefCell<StableCell<VaultId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(2))),
            0
        ).expect("failed to init NEXT_VAULT_ID")
    );

    /// This data structure stores metainformartion about what password IDs are
    /// stored in a vault. The password themselves and their metainformation are
    /// stored in `PASSWORDS` to be able to retrieve them separately.
    /// 
    /// A vault is accosicated with a vetkey. That is, all non-shared passwords
    /// in a vault are encrypted with different symmetric keys derivated from
    /// the same vetkey. Only vaults can be shared with other users, effectively
    /// sharing all passwords in the vault.
    static VAULTS: RefCell<StableBTreeMap<VaultId, Vault, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(3))),
        )
    );

    /// The status of a vault. A vault can be either up-to-date or require
    /// reencryption. The latter happens after a key rotation or access rights
    /// change. The status may hold auxiliary information for what to do after
    /// reencryption. Currently, this information is used to add a user after
    /// the vault is reencrypted because this cannot be done before
    /// reencryption.
    static VAULT_STATUS: RefCell<StableBTreeMap<VaultId, VaultStatus, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(4))),
    ));

    /// The access rights of users to vaults. This is a convenience map to be
    /// able to find all vaults accessible by a user quickly.
    static ACCESS: RefCell<StableBTreeMap<PrincipalName, VaultIds, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(5))),
        )
    );
}

/// Unlike Motoko, the caller identity is not built into Rust.
/// Thus, we use the ic_cdk::caller() method inside this wrapper function.
/// The wrapper prevents the use of the anonymous identity. Forbidding anonymous
/// interactions is the recommended default behavior for IC canisters.
fn caller() -> Principal {
    let caller = ic_cdk::caller();
    // The anonymous principal is not allowed to interact with the
    // encrypted notes canister.
    if caller == Principal::anonymous() {
        panic!("Anonymous principal is not allowed to make calls.")
    }
    caller
}

/// --- Queries vs. Updates ---
///
/// Note that our public methods are declared as an *updates* rather than *queries*, e.g.:
/// #[update(name = "notesCnt")] ...
/// rather than
/// #[query(name = "notesCnt")] ...
///
/// While queries are significantly faster than updates, they are not certified by the IC.
/// Thus, we avoid using queries throughout this dapp, ensuring that the result of our
/// methods gets through consensus. Otherwise, this method could e.g. omit some notes
/// if it got executed by a malicious node. (To make the dapp more efficient, one could
/// use an approach in which both queries and updates are combined.)
///
/// See https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods

/// Reflects the [caller]'s identity by returning (a future of) its principal.
/// Useful for debugging.
#[update]
fn whoami() -> String {
    ic_cdk::caller().to_string()
}

#[update]
fn get_accessible_vault_ids() -> Vec<VaultId> {
    let user_str = caller().to_string();
    ACCESS.with_borrow(|access| {
        if let Some(vault_ids) = access.get(&user_str) {
            vault_ids.vault_ids
        } else {
            vec![]
        }
    })
}

#[update]
fn create_vault() -> VaultId {
    let user_str = caller().to_string();

    // number of authorized vaults per user is limited
    if ACCESS.with_borrow(|access| access.len() as u64 >= MAX_ACCESS_VAULTS_PER_USER as u64) {
        ic_cdk::trap(format!("maximum number of authorized vaults per user reached: {MAX_ACCESS_VAULTS_PER_USER}").as_str());
    }

    let vault_id = NEXT_VAULT_ID.with_borrow_mut(|id| {
        let tmp = *id.get();
        id.set(tmp + 1)
            .unwrap_or_else(|_e| ic_cdk::trap("failed to set NEXT_VAULT_ID"));
        tmp
    });

    let new_vault = Vault {
        creator: user_str.clone(),
        version: 0,
        access: vec![],
        passwords: vec![],
        id: vault_id,
    };

    let old_vault = VAULTS.with_borrow_mut(|vaults| vaults.insert(vault_id, new_vault));
    assert!(
        old_vault.is_none(),
        "bug: vault was created with a fresh id and should not exist"
    );

    let old_vault_status = VAULT_STATUS
        .with_borrow_mut(|vault_status| vault_status.insert(vault_id, VaultStatus::UpToDate));
    assert!(
        old_vault_status.is_none(),
        "bug: vault status was created with a fresh id and should not exist"
    );

    ACCESS.with_borrow_mut(|access| {
        let mut user_access = access
            .get(&user_str)
            .unwrap_or(VaultIds { vault_ids: vec![] });
        user_access.vault_ids.push(vault_id);
        access.insert(user_str, user_access);
    });

    vault_id
}

/// Returns the vault version and the encrypted passwords of the vault or panics
/// if the vault does not exist or a user is unauthorized.
#[update]
fn get_vault(vault_id: VaultId) -> (VaultVersion, Vec<EncryptedPassword>) {
    let user_str = caller().to_string();

    VAULTS.with_borrow(|vaults| {
        if !(vaults.contains_key(&vault_id)) {
            ic_cdk::trap("vault_id does not exist");
        }
    });

    // fetch the passwords from own vault, fetch the passwords shared with the user
    let mut password_ids = vec![];
    let vault_version = ACCESS.with_borrow(|access| {
        let vault_ids = access.get(&user_str).expect("unauthorized user");
        VAULTS.with_borrow(|vaults| {
            if !vault_ids.vault_ids.contains(&vault_id) {
                ic_cdk::trap("unauthorized user");
            }
            let mut vault = vaults.get(&vault_id).expect("bug: failed to get vault");
            password_ids.append(&mut vault.passwords);
            vault.version
        })
    });

    let passwords = PASSWORDS.with_borrow(|passwords| {
        password_ids
            .into_iter()
            .map(|password_id| {
                passwords
                    .get(&password_id)
                    .expect("failed to get password")
                    .clone()
            })
            .collect()
    });

    (vault_version, passwords)
}

/// Reencrypts the vault with the given `vault_id` with the given
/// `reencrypted_passwords`.
/// 
/// * The reencrypted passwords are checked for consistency, i.e., the number of
/// passwords in the vault must match the number of reencrypted passwords, but
/// the validity of the passwords is not checked by the canister.
/// * The vault version must match the current vault version.
/// * The vault must be locked by changing access rights or invoking the key
///   rotation API.
#[update]
fn reencrypt_vault(
    vault_id: VaultId,
    current_vault_version: VaultVersion,
    reencrypted_passwords: Vec<String>,
) {
    let user_str = caller().to_string();

    VAULTS.with_borrow_mut(|vaults| {
        let mut vault = vaults.get(&vault_id).expect("failed to get vault");
        if !vault.can_manage(&user_str) {
            ic_cdk::trap("unauthorized user");
        }

        VAULT_STATUS.with_borrow(|vault_status| {
            match vault_status
                .get(&vault_id)
                .expect("bug: missing vault status")
            {
                VaultStatus::UpToDate => ic_cdk::trap(
                    "The vault is NOT locked but needs to be to make `reencrypt_vault` safer - \
                    even a simple key rotation should lock the vault to prevent overwriting \
                    data or calls that resulted from the misunderstanding that this API has \
                    more than two purposes, which are key updates due to 1) access rights \
                    changes and 2) key rotations.",
                ),
                VaultStatus::RequiresReencryption(_) => {}
            }
        });

        if vault.version != current_vault_version {
            ic_cdk::trap(
                "vault version mismatch, someone likely has modified the vault in the meantime",
            );
        }

        if vault.passwords.len() != reencrypted_passwords.len() {
            ic_cdk::trap("number of passwords in the vault does not match the number of reencrypted passwords");
        }

        vault.version += 1;

        PASSWORDS.with_borrow_mut(|passwords| {
            for (password_id, reencrypted_text) in vault.passwords.clone().into_iter().zip(reencrypted_passwords){
                    let mut password = passwords.get(&password_id).expect("bug: failed to get password");
                    password.encrypted_text = reencrypted_text;
                    password.password_version += 1;
                    
                    let old_password = passwords.insert(password_id, password);
                    assert!(old_password.is_some(), "bug: password should have existed");
            }
        });

        let old_vault = vaults.insert(vault_id, vault);
        assert!(old_vault.is_some(), "bug: vault should have existed");
    });

    VAULT_STATUS.with_borrow_mut(|vault_status| {
        let old_status = vault_status.insert(vault_id, VaultStatus::UpToDate);
        match old_status {
            None => ic_cdk::trap("bug: missing vault status"),
            Some(VaultStatus::UpToDate) => ic_cdk::trap("bug: reencrypted unlocked vault"),
            Some(VaultStatus::RequiresReencryption(opt_access_change)) => {
                match opt_access_change {
                    None => {
                        // key rotation, no modifications to user access required
                    }
                    Some(AccessChange::AddUser(user, access_rights)) => {
                        // the user can now be added after reencryption
                        ACCESS.with_borrow_mut(|access| {
                            let mut user_access = access.get(&user).unwrap_or(VaultIds { vault_ids: vec![] });
                            user_access.vault_ids.push(vault_id);
                            access.insert(user.clone(), user_access);
                        });
                        VAULTS.with_borrow_mut(|vaults| {
                            // the existence of the vault is already checked in `add_user`
                            let mut vault = vaults.get(&vault_id).expect("failed to get vault");
                            vault.access.push((user, access_rights));
                            let old_vault = vaults.insert(vault_id, vault);
                            assert!(old_vault.is_some(), "bug: vault should have existed");
                        });
                    }
                    Some(AccessChange::RemoveUser(_user)) => {
                        // the user was already removed
                    }
                }
            }
        }
    });
}

#[update]
fn delete_vault(vault_id: VaultId) {
    let user_str = caller().to_string();

    VAULTS.with_borrow_mut(|vaults| {
        let vault = vaults.get(&vault_id).expect("vault_id does not exist");
        if !vault.can_manage(&user_str) {
            ic_cdk::trap("unauthorized user");
        }
        let old_vault = vaults.remove(&vault_id);
        assert!(old_vault.is_some(), "bug: vault should have existed");
        ACCESS.with_borrow_mut(|access| {
            for user in vault.access.iter().map(|(u, _a)| u).chain(std::iter::once(&vault.creator)) {
                let mut user_access = access.get(user).expect("failed to get user access");
                assert!(user_access.vault_ids.contains(&vault_id));
                user_access.vault_ids.retain(|id| id != &vault_id);
                assert!(!user_access.vault_ids.contains(&vault_id));
                access.insert(user.clone(), user_access);
            }
        });
        PASSWORDS.with_borrow_mut(|passwords| {
            for password_id in vault.passwords.iter() {
                assert!(
                    passwords.remove(password_id).is_some(),
                    "bug: password should have existed"
                );
            }
        });
    });

    VAULT_STATUS.with_borrow_mut(|status| {
        assert!(status.remove(&vault_id).is_some(), "bug: vault status should have existed");
    })
}

#[update]
fn get_password(password_id: PasswordId) -> EncryptedPassword {
    let user_str = caller().to_string();
    let password = PASSWORDS.with_borrow(|passwords| {
        passwords
            .get(&password_id)
            .unwrap_or_else(|| ic_cdk::trap("password_id does not exist"))
    });

    ACCESS.with_borrow(|access| {
        let authorized_vault_ids = access
            .get(&user_str)
            .unwrap_or_else(|| ic_cdk::trap("unauthorized user"));
        if authorized_vault_ids
            .vault_ids
            .contains(&password.owner_vault_id)
        {
            password
        } else {
            ic_cdk::trap("unauthorized user")
        }
    })
}

/// Delete this [caller]'s note with given id. If none of the
/// existing notes have this id, do nothing.
/// [id]: the id of the note to be deleted
///
/// Returns:
///      Future of unit
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn delete_password(password_id: PasswordId) {
    let user_str = caller().to_string();

    PASSWORDS.with_borrow_mut(|passwords| {
        if let Some(password) = passwords.get(&password_id) {
            let vault_id = password.owner_vault_id;
            VAULTS.with_borrow_mut(|vaults| {
                let mut vault = vaults.get(&vault_id).expect("bug: failed to get vault");
                if !vault.can_write(&user_str) {
                    ic_cdk::trap("unauthorized user");
                }

                utils::expect_vault_not_locked(&vault_id);

                let count_before_deletion = vault.passwords.len();
                vault.passwords = vault
                    .passwords
                    .into_iter()
                    .filter(|this_id| this_id != &password_id)
                    .collect();
                assert_eq!(
                    vault.passwords.len() + 1,
                    count_before_deletion,
                    "bug: no password id in the vault"
                );
                let old_vault = vaults.insert(vault_id, vault);
                assert!(
                    old_vault.is_some(),
                    "bug: overwritten vault should have exist"
                );
                assert!(
                    passwords.remove(&password_id).is_some(),
                    "bug: password should have existed"
                );
            });
        } else {
            ic_cdk::trap("password_id does not exist");
        }
    });
}

/// Replaces the encrypted text of note with ID [id] with [encrypted_text].
///
/// Panics:
///     [caller] is the anonymous identity
///     [caller] is not the note's owner and not a user with whom the note is shared
///     [encrypted_text] exceeds [MAX_VAULT_CHARS]
#[update]
fn update_password(
    password_id: PasswordId,
    current_password_version: PasswordVersion,
    encrypted_text: String,
) {
    let user_str = caller().to_string();

    PASSWORDS.with_borrow_mut(|passwords| {
        let mut password_to_update = passwords.get(&password_id).unwrap_or_else(|| ic_cdk::trap("password_id does not exist"));
            let vault = VAULTS.with_borrow(|vaults| vaults.get(&password_to_update.owner_vault_id).unwrap_or_else(|| ic_cdk::trap("Didn't find vault for the password")));

            if !vault.can_write(&user_str) {
                ic_cdk::trap("unauthorized user");
            }

            utils::expect_vault_not_locked(&password_to_update.owner_vault_id);

            if password_to_update.password_version != current_password_version {
                ic_cdk::trap("password version mismatch, someone likely has modified the password in the meantime");
            }
            password_to_update.password_version += 1;
            assert!(encrypted_text.chars().count() <= MAX_PASSWORD_CHARS, "encrypted message is to large: expected at most {MAX_PASSWORD_CHARS} chars, but got {}", encrypted_text.chars().count());
            password_to_update.encrypted_text = encrypted_text;
            passwords.insert(password_id, password_to_update);
        });
}

/// Add new empty note for this [caller].
///
/// Returns:
///      Future of ID of new empty note
/// Panics:
///      [caller] is the anonymous identity
///      User already has [MAX_VAULTS_PER_USER] notes
///      This is the first note for [caller] and [MAX_USERS] is exceeded
#[update]
fn create_password(vault_id: VaultId) -> PasswordId {
    let mut vault = VAULTS.with_borrow(|vaults| {
        vaults
            .get(&vault_id)
            .unwrap_or_else(|| ic_cdk::trap("Vault does not exist"))
    });

    let caller = caller().to_string();

    if !vault.can_write(&caller) {
        ic_cdk::trap("unauthorized user");
    };

    utils::expect_vault_not_locked(&vault_id);

    if utils::number_of_passwords_of_user(&caller) >= MAX_PASSWORDS_PER_USER {
        ic_cdk::trap(
            format!("User already has MAX_PASSWORDS_PER_USER={MAX_PASSWORDS_PER_USER} notes")
                .as_str(),
        );
    }

    let password_id = NEXT_PASSWORD_ID.with_borrow_mut(|id| {
        let tmp = *id.get();
        id.set(tmp + 1)
            .unwrap_or_else(|_e| ic_cdk::trap("failed to set NEXT_VAULT_ID"));
        tmp
    });
    let new_password = EncryptedPassword {
        owner_vault_id: vault_id,
        password_id,
        encrypted_text: String::new(),
        password_version: 0,
    };

    vault.passwords.push(password_id);

    let old_vault = VAULTS.with_borrow_mut(|vaults| vaults.insert(vault_id, vault));
    assert!(
        old_vault.is_some(),
        "The vault we are creating the password in should have existed"
    );

    let old_password =
        PASSWORDS.with_borrow_mut(|passwords| passwords.insert(password_id, new_password));
    assert!(
        old_password.is_none(),
        "password was created with a fresh id and should not exist"
    );

    password_id
}

/// Shares the note with ID `note_id`` with the `user`.
/// Has no effect if the note is already shared with that user.
///
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn add_user(vault_id: VaultId, user: PrincipalName, access_rights: AccessRights) {
    let caller_str = caller().to_string();
    ACCESS.with_borrow(|access| {
        let caller_access_to_vault_ids = access
            .get(&caller_str)
            .unwrap_or(VaultIds { vault_ids: vec![] });
        if !caller_access_to_vault_ids.vault_ids.contains(&vault_id) {
            ic_cdk::trap("unauthorized user");
        }

        if access.len() as u64 >= MAX_USERS {
            ic_cdk::trap(format!("maximum number of users reached: {}", MAX_USERS).as_str());
        }

        let mut user_access = access.get(&user).unwrap_or(VaultIds { vault_ids: vec![] });

        VAULTS.with_borrow(|vaults| {
            let vault = vaults.get(&vault_id).expect("failed to get vault");
            if !vault.can_manage(&caller_str) {
                ic_cdk::trap("unauthorized user");
            }

            if user_access.vault_ids.contains(&vault_id) || vault.creator == user {
                ic_cdk::trap("user already has access to the vault");
            }
            user_access.vault_ids.push(vault_id);

            if vault.access.iter().any(|(p, _a)| p == &user) {
                ic_cdk::trap("bug: user already has access to the vault");
            }

            if vault.access.len() >= MAX_SHARES_PER_VAULT {
                ic_cdk::trap(
                    "maximum number of shared users for a vault reached: {MAX_SHARES_PER_VAULT}",
                );
            };
        });
    });

    utils::lock_vault(vault_id, Some(vault::AccessChange::AddUser(user, access_rights)));
}

/// Unshares the note with ID `note_id`` with the `user`.
/// Has no effect if the note is not shared with that user.
///
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn remove_user(vault_id: VaultId, user: PrincipalName) {
    let caller_str = caller().to_string();

    ACCESS.with_borrow_mut(|access| {
        let caller_access_to_vault_ids = access
            .get(&caller_str)
            .unwrap_or(VaultIds { vault_ids: vec![] });
        if !caller_access_to_vault_ids.vault_ids.contains(&vault_id) {
            ic_cdk::trap("unauthorized user");
        }
        let mut user_access = access.get(&user).unwrap_or(VaultIds { vault_ids: vec![] });

        VAULTS.with_borrow_mut(|vaults| {
            let mut vault = vaults.get(&vault_id).expect("failed to get vault");
            if !vault.can_manage(&caller_str) {
                ic_cdk::trap("unauthorized user");
            }

            if vault.creator == user {
                ic_cdk::trap("cannot remove the creator of the vault");
            }

            if !user_access.vault_ids.contains(&vault_id) {
                ic_cdk::trap("to-be-removed user does not have access to the vault");
            }
            user_access.vault_ids.retain(|id| id != &vault_id);

            if !vault.access.iter().any(|(p, _a)| p == &user) {
                ic_cdk::trap("bug: user already has no access to the vault");
            }

            if vault.access.len() >= MAX_SHARES_PER_VAULT {
                ic_cdk::trap(
                    "maximum number of shared users for a vault reached: {MAX_SHARES_PER_VAULT}",
                );
            };

            vault.access.retain(|(p, _a)| p != &user);

            let old_vault = vaults.insert(vault_id, vault);
            assert!(old_vault.is_some(), "bug: vault should have existed");
        });

        let old_user_access = access.insert(user.clone(), user_access);
        assert!(
            old_user_access.is_some(),
            "bug: user should have existed"
        );
    });

    utils::lock_vault(vault_id, Some(vault::AccessChange::RemoveUser(user)));
}

#[update]
fn rotate_vault_key(vault_id: VaultId) {
    let user_str = caller().to_string();

    VAULTS.with_borrow_mut(|vaults| {
        let vault = vaults.get(&vault_id).expect("failed to get vault");
        if !vault.can_write(&user_str) {
            ic_cdk::trap("unauthorized user");
        }

        utils::expect_vault_not_locked(&vault_id);
    });

    utils::lock_vault(vault_id, None);
}

/// Verification key for the password manager that can be used to verify the
/// symmetric keys related to vaults.
#[update]
async fn verification_key_for_vault() -> String {
    let request = VetKDPublicKeyRequest {
        canister_id: None,
        derivation_path: utils::derivation_path(),
        key_id: utils::bls12_381_test_key_1(),
    };

    let (response,): (VetKDPublicKeyReply,) = ic_cdk::call(
        utils::vetkd_system_api_canister_id(),
        "vetkd_public_key",
        (request,),
    )
    .await
    .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_symmetric_key_for_vault(
    vault_id: VaultId,
    encryption_public_key: Vec<u8>,
) -> (VaultVersion, String, Vec<u8>) {
    use ic_stable_structures::Storable;

    let user_str = caller().to_string();
    let (vault_version, request) = VAULTS.with_borrow(|vaults| {
        if let Some(vault) = vaults.get(&vault_id) {
            if !vault.can_read(&user_str) {
                ic_cdk::trap("unauthorized user");
            }
            (
                vault.version,
                VetKDEncryptedKeyRequest {
                    derivation_id: {
                        let mut buf = vec![];
                        buf.extend_from_slice(&vault_id.to_bytes()); // fixed-size encoding
                        buf.extend_from_slice(vault.version.to_be_bytes().as_slice());
                        buf // prefix-free
                    },
                    public_key_derivation_path: utils::derivation_path(),
                    key_id: utils::bls12_381_test_key_1(),
                    encryption_public_key,
                },
            )
        } else {
            ic_cdk::trap(&format!("vault with ID {vault_id} does not exist"));
        }
    });

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::call(
        utils::vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request.clone(),),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    (vault_version, hex::encode(response.encrypted_key), request.derivation_id)
}

#[update]
async fn next_encrypted_symmetric_key_for_locked_vault(
    vault_id: VaultId,
    encryption_public_key: Vec<u8>,
) -> (VaultVersion, String) {
    use ic_stable_structures::Storable;

    VAULT_STATUS.with_borrow(|vault_status| match vault_status.get(&vault_id) {
        None => {
            ic_cdk::trap("vault not found");
        }
        Some(VaultStatus::UpToDate) => {
            ic_cdk::trap("next key for vault only makes sense for reencryption, but the target vault is not locked");
        }
        Some(VaultStatus::RequiresReencryption(_)) => {}
    });

    let user_str = caller().to_string();
    let (next_vault_version, request) = VAULTS.with_borrow(|vaults| {
        if let Some(vault) = vaults.get(&vault_id) {
            if !vault.can_read(&user_str) {
                ic_cdk::trap("unauthorized user");
            }
            let next_vault_version = vault.version + 1;
            (
                next_vault_version,
                VetKDEncryptedKeyRequest {
                    derivation_id: {
                        let mut buf = vec![];
                        buf.extend_from_slice(&vault_id.to_bytes()); // fixed-size encoding
                        buf.extend_from_slice(next_vault_version.to_be_bytes().as_slice());
                        buf // prefix-free
                    },
                    public_key_derivation_path: utils::derivation_path(),
                    key_id: utils::bls12_381_test_key_1(),
                    encryption_public_key,
                },
            )
        } else {
            ic_cdk::trap(&format!("vault with ID {vault_id} does not exist"));
        }
    });

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::call(
        utils::vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request,),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    (next_vault_version, hex::encode(response.encrypted_key))
}

pub mod utils {
    use vault::AccessChange;

    use super::*;

    pub fn number_of_passwords_of_user(user: &PrincipalName) -> usize {
        ACCESS.with_borrow(|access| {
            let access_to_vauld_ids = access.get(user).unwrap_or(VaultIds { vault_ids: vec![] });
            VAULTS.with_borrow(|vaults| {
                let mut count = 0;
                for vault_id in access_to_vauld_ids.vault_ids.iter() {
                    let vault = vaults.get(vault_id).expect("failed to get vault");
                    count += vault.passwords.len();
                }
                count
            })
        })
    }

    pub fn lock_vault(vault_id: VaultId, opt_access_change: Option<AccessChange>) {
        VAULT_STATUS.with_borrow_mut(|vault_status| {
            let old_status = match vault_status.get(&vault_id) {
                None => {
                    ic_cdk::trap("bug: missing vault status");
                }
                Some(VaultStatus::UpToDate) => {
                    vault_status.insert(vault_id, VaultStatus::RequiresReencryption(opt_access_change))
                }
                Some(VaultStatus::RequiresReencryption(_)) => {
                    ic_cdk::trap("vault is already locked for reencryption");
                }
            };
            assert!(
                old_status.is_some(),
                "bug: vault status should have existed"
            );
        });
    } 

    pub fn expect_vault_not_locked(vault_id: &VaultId) {
        VAULT_STATUS.with_borrow(|vault_status| {
            match vault_status
                .get(&vault_id)
                .expect("bug: missing vault status")
            {
                VaultStatus::UpToDate => {}
                VaultStatus::RequiresReencryption(_) => {
                    ic_cdk::trap("a user requested a vault reencryption likely due to access rights change or key rotation - the vault is not writable until the reencryption is complete via an reencrypt_vault call");
                }
            }
        });
    }

    pub fn bls12_381_test_key_1() -> VetKDKeyId {
        VetKDKeyId {
            curve: VetKDCurve::Bls12_381,
            name: "test_key_1".to_string(),
        }
    }

    pub fn vetkd_system_api_canister_id() -> CanisterId {
        use std::str::FromStr;
        CanisterId::from_str(VETKD_SYSTEM_API_CANISTER_ID).expect("failed to create canister ID")
    }

    pub fn derivation_path() -> Vec<Vec<u8>> {
        vec![b"password_manager".to_vec()]
    }
}

ic_cdk_macros::export_candid!();
