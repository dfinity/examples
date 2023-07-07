use candid::CandidType;
use ic_cdk::api::caller as caller_api;
use ic_cdk::export::{candid, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem;

type PrincipalName = String;

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
    // The data is reserved for storing the notes:
    //     NOTES_PER_USER = MAX_NOTES_PER_USER x MAX_NOTE_CHARS x (4 bytes per char)
    //     2 MB = 500 x 1000 x 4 = 2,000,000

    // Define dapp limits - important for security assurance
    static MAX_USERS: usize = 1_000;
    static MAX_NOTES_PER_USER: usize = 500;
    static MAX_NOTE_CHARS: usize = 1000;

    pub static NEXT_NOTE: RefCell<u128> = RefCell::new(1);
    pub static NOTES_BY_USER: RefCell<BTreeMap<PrincipalName, Vec<EncryptedNote>>> = RefCell::new(BTreeMap::new());
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
    assert!(note.encrypted_text.chars().count() <= MAX_NOTE_CHARS.with(|mnc| *mnc));
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

/// Add new note for this [caller].
///      [note]: (encrypted) content of this note
///
/// Returns: 
///      Future of unit
/// Panics: 
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [note] exceeds [MAX_NOTE_CHARS]
///      User already has [MAX_NOTES_PER_USER] notes
///      [note] would be for a new user and [MAX_USERS] is exceeded
#[update(name = "add_note")]
fn add_note(note: String) {
    let user = caller();
    assert!(note.chars().count() <= MAX_NOTE_CHARS.with(|mnc| *mnc));

    let user_str = user.to_string();
    let note_id = NEXT_NOTE.with(|counter_ref| {
        let mut writer = counter_ref.borrow_mut();
        *writer += 1;
        *writer
    });

    let user_count = user_count();
    NOTES_BY_USER.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        let user_notes = writer.entry(user_str).or_insert_with(|| {
            // caller unknown ==> check invariants
            // A. can we add a new user?
            assert!(MAX_USERS.with(|mu| user_count < *mu));
            vec![]
        });

        assert!(user_notes.len() < MAX_NOTES_PER_USER.with(|mnpu| *mnpu));

        user_notes.push(EncryptedNote {
            id: note_id,
            encrypted_text: note,
        });
    });
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
            let old_state = CanisterState {
                notes: mem::take(&mut notes_ref.borrow_mut()),
                counter: copied_counter,
            };
            // storage::stable_save is the API used to write canister state out.
            // More explicit error handling *can* be useful, but if we fail to read out/in stable memory on upgrade
            // it means the data won't be accessible to the canister in any way.
            storage::stable_save((old_state,)).unwrap();
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
                *notes_ref.borrow_mut() = old_state.notes;
                *counter_ref.borrow_mut() = old_state.counter;
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
}

mod vetkd_types;

const VETKD_SYSTEM_API_CANISTER_ID: &str = "br5f7-7uaaa-aaaaa-qaaca-cai";

use vetkd_types::{
    CanisterId, VetKDCurve, VetKDEncryptedKeyReply, VetKDEncryptedKeyRequest, VetKDKeyId,
    VetKDPublicKeyReply, VetKDPublicKeyRequest,
};

/// Results can be cached.
#[update]
async fn app_vetkd_public_key(derivation_path: Vec<Vec<u8>>) -> String {
    let request = VetKDPublicKeyRequest {
        canister_id: None,
        derivation_path,
        key_id: bls12_381_test_key_1(),
    };

    let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_public_key",
        (request,),
    )
    .await
    .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_symmetric_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    let request = VetKDEncryptedKeyRequest {
        derivation_id: ic_cdk::caller().as_slice().to_vec(),
        public_key_derivation_path: vec![b"symmetric_key".to_vec()],
        key_id: bls12_381_test_key_1(),
        encryption_public_key,
    };

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request,),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    hex::encode(response.encrypted_key)
}

fn bls12_381_test_key_1() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381,
        name: "test_key_1".to_string(),
    }
}

fn vetkd_system_api_canister_id() -> CanisterId {
    use std::str::FromStr;
    CanisterId::from_str(VETKD_SYSTEM_API_CANISTER_ID).expect("failed to create canister ID")
}
