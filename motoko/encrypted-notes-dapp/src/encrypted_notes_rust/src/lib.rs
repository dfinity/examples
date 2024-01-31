use candid::CandidType;
use ic_cdk::api::{caller as caller_api};
use ic_cdk::export::{candid, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem;
use std::collections::btree_map::Entry::{Vacant, Occupied};

type PrincipalName = String;
type PublicKey = String;
type Ciphertext = String;
type DeviceAlias = String;

#[derive(Clone, CandidType, Deserialize)]
pub enum GetCiphertextError {
    // Ensure API types in [encrypted_notes_rust.did] are named exactly as specified below
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "notSynced")]
    NotSynced,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum Result {
    // Ensure API types in [encrypted_notes_rust.did] are named exactly as specified below
    #[serde(rename = "err")]
    Err(GetCiphertextError),
    #[serde(rename = "ok")]
    Ok(Ciphertext),
}

/// Deriving CandidType or implementing it is necessary for
/// almost everything IC - if you want your structs to
/// Save in stable storage or serialize in inputs/outputs
/// You should derive CandidType, Serialize, Deserialize.
#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct EncryptedNote {
    id: u128,
    encrypted_text: String,
}

/// There can only be one Type in stable storage at a time.
/// We use this struct to represent the full CanisterState
/// So we can serialize it to stable storage.
#[derive(Clone, CandidType, Serialize, Deserialize)]
struct CanisterState {
    // During canister upgrades, this field contains a stable representation of the value stored in [NEXT_NOTE]
    counter: u128,
    // We use a BTreeMap vice a HashMap for deterministic ordering.
    notes: BTreeMap<PrincipalName, Vec<EncryptedNote>>,
    user_store: BTreeMap<Principal, UserStore>,
}

#[derive(Clone, CandidType, Serialize, Deserialize, Default)]
pub struct UserStore {
    // We use a BTreeMap vice a HashMap for deterministic ordering.
    device_list: BTreeMap<DeviceAlias, PublicKey>,
    ciphertext_list: BTreeMap<PublicKey, Ciphertext>,
}

impl UserStore {
    pub fn new() -> Self {
        Self::default()
    }
}

// WASM is single-threaded by nature. [RefCell] and [thread_local!] are used despite being not totally safe primitives.
// This is to ensure that the canister state can be used throughout.
// Your other option here is to avoid [thread_local!] and use a [RefCell<RwLock/Mutex/Atomic>].
// Here we use [thread_local!] because it is simpler.
thread_local! {

    // Currently, a single canister smart contract is limited to 4 GB of storage due to WebAssembly limitations.
    // To ensure that our canister does not exceed this limit, we restrict memory usage to at most 2 GB because 
    // up to 2x memory may be needed for data serialization during canister upgrades. Therefore, we aim to support
    // up to 1,000 users, each storing up to 2 MB of data. 
    // 1) One half of this data is reserved for device management: 
    //     DEVICES_PER_USER = (MAX_CYPHERTEXT_LENGTH + MAX_PUBLIC_KEY_LENGTH + MAX_DEVICE_ALIAS_LENGTH) x (4 bytes per char) x MAX_DEVICES_PER_USER
    //     1 MB = 40,700 x 4 x 6 = 976,800
    // 2) Another half is reserved for storing the notes:
    //     NOTES_PER_USER = MAX_NOTES_PER_USER x MAX_NOTE_CHARS x (4 bytes per char)
    //     1 MB = 500 x 500 x 4 = 1,000,000

    // Define dapp limits - important for security assurance
    static MAX_USERS: usize = 1_000;
    static MAX_NOTES_PER_USER: usize = 500;
    static MAX_DEVICES_PER_USER: usize = 6;
    static MAX_NOTE_CHARS: usize = 500;
    static MAX_DEVICE_ALIAS_LENGTH: usize = 200;
    static MAX_PUBLIC_KEY_LENGTH: usize = 500;
    static MAX_CYPHERTEXT_LENGTH: usize = 40_000;

    pub static NEXT_NOTE: RefCell<u128> = RefCell::new(1);
    pub static NOTES_BY_USER: RefCell<BTreeMap<PrincipalName, Vec<EncryptedNote>>> = RefCell::new(BTreeMap::new());
    pub static USER_KEYS: RefCell<BTreeMap<Principal, UserStore>> = RefCell::new(BTreeMap::new());
}

/// Unlike Motoko, the caller identity is not built into Rust. 
/// Thus, we use the ic_cdk::api::caller() method inside this wrapper function.
/// The wrapper prevents the use of the anonymous identity. Forbidding anonymous 
/// interactions is the recommended default behavior for IC canisters. 
fn caller() -> Principal {
    let caller = caller_api();
    // The anonymous principal is not allowed to interact with the 
    // encrypted notes canister.
    if caller == Principal::anonymous() {
        panic!("Anonymous principal not allowed to make calls.")
    }
    caller
}

#[init]
fn init() {}

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
#[update(name = "whoami")]
fn whoami() -> String {
    caller_api().to_string()
}

/// General assumptions
/// -------------------
/// All the functions of this canister's public API should be available only to
/// registered users, with the exception of [register_device] and [whoami].

/// Returns the current number of users.
fn user_count() -> usize {
    NOTES_BY_USER.with(|notes_ref| notes_ref.borrow().keys().len())
}

/// Check if this user has been registered
/// Note: [register_device] must be each user's very first update call.
fn is_user_registered(principal: Principal) -> bool {
    USER_KEYS.with(|user_keys_ref| user_keys_ref.borrow().contains_key(&principal))
}

/// Check that a note identifier is sane. This is needed since we use finite-
/// precision integers (`u128`).
fn is_id_sane(id: u128) -> bool {
    MAX_NOTES_PER_USER.with(|max_notes_per_user| id < (*max_notes_per_user as u128) * (user_count() as u128))
}

/// Returns (a future of) this [caller]'s notes.
/// Panics: 
///     [caller] is the anonymous identity
///     [caller] is not a registered user
#[update(name = "get_notes")]
fn get_notes() -> Vec<EncryptedNote> {
    let user = caller();
    assert!(is_user_registered(user));
    let user_str = user.to_string();
    NOTES_BY_USER.with(|notes_ref| {
        notes_ref
            .borrow()
            .get(&user_str)
            .cloned()
            .unwrap_or_default()
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
///      [caller] is not a registered user
///      [id] is unreasonable; see [is_id_sane]
#[update(name = "delete_note")]
fn delete_note(note_id: u128) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(is_id_sane(note_id));

    let user_str = user.to_string();
    // shared ownership borrowing
    NOTES_BY_USER.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        if let Some(v) = writer.get_mut(&user_str) {
            v.retain(|item| item.id != note_id);
        }
    });
}

/// Returns (a future of) this [caller]'s notes.
/// Panics: 
///     [caller] is the anonymous identity
///     [caller] is not a registered user
///     [note.encrypted_text] exceeds [MAX_NOTE_CHARS]
///     [note.id] is unreasonable; see [is_id_sane]
#[update(name = "update_note")]
fn update_note(note: EncryptedNote) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(note.encrypted_text.chars().count() <= MAX_NOTE_CHARS.with(|mnc| mnc.clone()));
    assert!(is_id_sane(note.id));

    let user_str = user.to_string();
    NOTES_BY_USER.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        if let Some(old_note) = writer
            .get_mut(&user_str)
            .and_then(|notes| notes.iter_mut().find(|n| n.id == note.id))
        {
            old_note.encrypted_text = note.encrypted_text;
        }
    })
}

/// Add new note for this [caller]. Note: this function may be called only by 
/// those users that have at least one device registered via [register_device].
///      [note]: (encrypted) content of this note
///
/// Returns: 
///      Future of unit
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [note] exceeds [MAX_NOTE_CHARS]
///      User already has [MAX_NOTES_PER_USER] notes
#[update(name = "add_note")]
fn add_note(note: String) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(note.chars().count() <= MAX_NOTE_CHARS.with(|mnc| mnc.clone()));

    let user_str = user.to_string();
    let note_id = NEXT_NOTE.with(|counter_ref| {
        let mut writer = counter_ref.borrow_mut();
        *writer += 1;
        *writer
    });

    NOTES_BY_USER.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        let user_notes = writer.get_mut(&user_str)
            .expect(&format!("detected registered user {} w/o allocated notes", user_str)[..]);
        
        assert!(user_notes.len() < MAX_NOTES_PER_USER.with(|mnpu| mnpu.clone()));

        user_notes.push(EncryptedNote {
            id: note_id,
            encrypted_text: note,
        });
    });
}

/// Associate a public key with a device ID.
/// Returns: 
///      `true` iff device is *newly* registered, ie. [alias] has not been 
///      registered for this user before. 
/// Panics:
///      [caller] is the anonymous identity
///      [alias] exceeds [MAX_DEVICE_ALIAS_LENGTH]
///      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
///      While registering new user's device:
///          There are already [MAX_USERS] users while we need to register a new user
///          This user already has notes despite not having any registered devices
///      This user already has [MAX_DEVICES_PER_USER] registered devices.
#[update(name = "register_device")]
fn register_device(alias: DeviceAlias, pk: PublicKey) -> bool {

    let caller = caller();
    assert!(MAX_DEVICE_ALIAS_LENGTH.with(|mdal| alias.len() <= mdal.clone()));
    assert!(MAX_PUBLIC_KEY_LENGTH.with(|mpkl| pk.len() <= mpkl.clone()));
    
    USER_KEYS.with(|user_keys_ref| {
        let mut writer = user_keys_ref.borrow_mut();
        match writer.entry(caller) {
            Vacant(empty_store_entry) => {
                // caller unknown ==> check invariants
                // A. can we add a new user?
                assert!(MAX_USERS.with(|mu| user_count() < mu.clone()));
                // B. this caller does not have notes
                let principal_name = caller.to_string();
                assert!(NOTES_BY_USER.with(|notes_ref| !notes_ref.borrow().contains_key(&principal_name)));

                // ... then initialize the following:
                // 1) a new [UserStore] instance in [USER_KEYS]
                empty_store_entry.insert({
                    let mut dl = BTreeMap::new();
                    dl.insert(alias, pk);
                    UserStore {
                        device_list: dl,
                        ciphertext_list: BTreeMap::new(),
                    }
                });
                // 2) a new [Vec<EncryptedNote>] entry in [NOTES_BY_USER]
                NOTES_BY_USER.with(|notes_ref| 
                    notes_ref.borrow_mut().insert(principal_name, vec![]));
                
                // finally, indicate accept
                true
            },
            Occupied(mut store_entry) => {
                // caller is a registered user
                let store = store_entry.get_mut();
                let inv = MAX_DEVICES_PER_USER.with(|mdpu| store.device_list.len() < mdpu.clone());
                match store.device_list.entry(alias) {
                    Occupied(_) => {
                        // device alias already registered ==> indicate reject
                        false
                    },
                    Vacant(empty_device_entry) => {
                        // device not yet registered ==> check that user did not exceed limits
                        assert!(inv);
                        // all good ==> register device
                        empty_device_entry.insert(pk);
                        // indicate accept
                        true
                    }
                }
            }
        }
    })
}

/// Remove this user's device with given [alias]
///
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [alias] exceeds [MAX_DEVICE_ALIAS_LENGTH]
///      [caller] has only one registered device (which we refuse to remove)
#[update(name = "remove_device")]
fn remove_device(alias: DeviceAlias) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(MAX_DEVICE_ALIAS_LENGTH.with(|mdal| alias.len() <= mdal.clone()));

    USER_KEYS.with(|user_keys_ref| {
        let mut writer = user_keys_ref.borrow_mut();
        if let Some(user_store) = writer.get_mut(&user) {
            assert!(user_store.device_list.len() > 1);

            let pub_key = user_store.device_list.remove(&alias);
            if let Some(pk) = pub_key {
                // the device may or may not have an associated Ciphertext at this point
                user_store.ciphertext_list.remove(&pk);
            }
        }
    });
}

/// Returns:
///      Future vector of all (device, public key) pairs for this user's registered devices.
///
///      See also [get_notes] and "Queries vs. Updates"
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
#[update(name = "get_devices")]
fn get_devices() -> Vec<(DeviceAlias, PublicKey)> {
    let user = caller();
    assert!(is_user_registered(user));

    USER_KEYS.with(|user_keys_ref| {
        let reader = user_keys_ref.borrow_mut();
        match reader.get(&user) {
            Some(v) => {
                let out = v
                    .device_list
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<Vec<(DeviceAlias, PublicKey)>>();
                out
            }
            None => Vec::new(),
        }
    })
}

/// Returns:
///      Future vector of all public keys that are not already associated with a device.
///
///      See also [get_notes] and "Queries vs. Updates"
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
#[update(name = "get_unsynced_pubkeys")]
fn get_unsynced_pubkeys() -> Vec<PublicKey> {
    let user = caller();
    assert!(is_user_registered(user));

    USER_KEYS.with(|user_keys_ref| {
        let reader = user_keys_ref.borrow();
        reader.get(&caller()).map_or_else(Vec::new, |v| {
            v.device_list
                .values()
                .filter(|value| !v.ciphertext_list.contains_key(*value))
                .cloned()
                .collect::<Vec<PublicKey>>()
        })
    })
}

/// Returns: 
///      `true` iff the user has at least one public key.
///
///      See also [get_notes] and "Queries vs. Updates"
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
#[update(name = "is_seeded")]
fn is_seeded() -> bool {
    let user = caller();
    assert!(is_user_registered(user));

    USER_KEYS.with(|user_keys_ref| {
        let reader = user_keys_ref.borrow();
        reader
            .get(&user)
            .map_or(false, |v| !v.ciphertext_list.is_empty())
    })
}

/// Fetch the private key associated with this public key.
/// See also [get_notes] and "Queries vs. Updates"
/// Returns:
///      Future of Ciphertext result
/// Traps: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
#[update(name = "get_ciphertext")]
fn get_ciphertext(pk: PublicKey) -> Result {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(MAX_PUBLIC_KEY_LENGTH.with(|mpkl| pk.len() <= mpkl.clone()));
    
    USER_KEYS.with(|user_keys_ref| {
        let reader = user_keys_ref.borrow();
        if let Some(store) = reader.get(&user) {
            if !is_known_public_key(store, &pk) {
                Result::Err(GetCiphertextError::NotFound)
            } else if let Some(ciphertext) = store.ciphertext_list.get(&pk) {
                Result::Ok(ciphertext.to_string())
            } else {
                Result::Err(GetCiphertextError::NotSynced)
            }
        } else {
            Result::Err(GetCiphertextError::NotFound)
        }
    })
}

// Returns `true` iff [store.device_list] contains provided public key [pk].
fn is_known_public_key(store: &UserStore, pk: &str) -> bool {
    return store.device_list.values().any(|value| *value == pk);
}

/// Store a vector of public keys and associated private keys. 
/// Considers only public keys matching those of a registered device.
/// Does not overwrite key-value pairs that already exist.
///
/// Traps: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      Length of [Ciphertexts] exceeds [MAX_DEVICES_PER_USER]
///      User is trying to save a known device's Ciphertext exceeding [MAX_CYPHERTEXT_LENGTH]
#[update(name = "submit_ciphertexts")]
fn submit_ciphertexts(ciphertexts: Vec<(PublicKey, Ciphertext)>) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(MAX_DEVICES_PER_USER.with(|mdpu| ciphertexts.len() <= mdpu.clone()));

    USER_KEYS.with(|user_keys_ref| {
        let mut writer = user_keys_ref.borrow_mut();
        if let Some(store) = writer.get_mut(&user) {
            for (pk, ct) in ciphertexts {
                if is_known_public_key(store, &pk) {
                    assert!(MAX_CYPHERTEXT_LENGTH.with(|mcl| ct.len() <= mcl.clone()));
                    store.ciphertext_list.entry(pk).or_insert(ct);
                }
            }
        }
    })
}

/// Store a public key and associated private key in an empty user store. 
/// This function is a no-op if the user already has at least one public key stored.
///
/// Traps: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
///      [ct] exceeding [MAX_CYPHERTEXT_LENGTH]
#[update]
fn seed(pk: PublicKey, ct: Ciphertext) {
    let user = caller();
    assert!(is_user_registered(user));
    assert!(MAX_PUBLIC_KEY_LENGTH.with(|mpkl| pk.len() <= mpkl.clone()));
    assert!(MAX_CYPHERTEXT_LENGTH.with(|mcl| ct.len() <= mcl.clone()));

    USER_KEYS.with(|user_keys_ref| {
        let mut writer = user_keys_ref.borrow_mut();
        if let Some(store) = writer.get_mut(&user) {
            if is_known_public_key(store, &pk) && store.ciphertext_list.is_empty() {
                store.ciphertext_list.insert(pk, ct);
            }
        }
    })
}

/// Hooks in these macros will produce a `function already defined` error
/// if they share the same name as the underlying function.

#[pre_upgrade]
/// The pre_upgrade hook determines anything your canister
/// should do before it goes offline for a code upgrade.
fn pre_upgrade() {
    let copied_counter: u128 = NEXT_NOTE.with(|counter_ref| {
        let reader = counter_ref.borrow();
        *reader
    });
    NOTES_BY_USER.with(|notes_ref| {
        USER_KEYS.with(|user_keys_ref| {
            let old_state = CanisterState {
                notes: mem::take(&mut notes_ref.borrow_mut()),
                counter: copied_counter,
                user_store: mem::take(&mut user_keys_ref.borrow_mut()),
            };
            // storage::stable_save is the API used to write canister state out.
            // More explicit error handling *can* be useful, but if we fail to read out/in stable memory on upgrade
            // it means the data won't be accessible to the canister in any way.
            storage::stable_save((old_state,)).unwrap();
        })
    });
}

#[post_upgrade]
/// The post_upgrade hook determines anything your canister should do after it restarts
fn post_upgrade() {
    // storage::stable_restore is how to read your canister state back in from stable memory
    // Same thing with the unwrap here. For this canister there's nothing to do
    // in the event of a memory read out/in failure.
    let (old_state,): (CanisterState,) = storage::stable_restore().unwrap();
    NOTES_BY_USER.with(|notes_ref| {
        NEXT_NOTE.with(|counter_ref| {
            USER_KEYS.with(|user_keys_ref| {
                *notes_ref.borrow_mut() = old_state.notes;
                *counter_ref.borrow_mut() = old_state.counter;
                *user_keys_ref.borrow_mut() = old_state.user_store;
            })
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_count_succeeds() {
        assert_eq!(user_count(), 0);
    }

    #[test]
    fn test_is_user_registered_succeeds() {
        let is_registered = is_user_registered(Principal::anonymous());
        assert!(!is_registered);
    }
}
