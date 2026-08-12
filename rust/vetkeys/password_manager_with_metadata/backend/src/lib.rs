//! This canister layers a second, backend-authoritative store on top of the
//! `ic-vetkeys` EncryptedMaps library: every encrypted password has a matching
//! metadata row (creation date, modification counter, tags, url) that the
//! canister — not the client — maintains.
//!
//! The `custom_value_endpoints` form of the library macro generates the state,
//! the `#[init]`/`#[post_upgrade]`, the control-plane endpoints (vetKD keys,
//! access control, map-name enumeration) and the `with_encrypted_maps`/`_mut`
//! accessors, but *no* endpoints that read or write encrypted values. That is
//! what this canister needs: the raw `insert_encrypted_value` /
//! `remove_encrypted_value` mutators are never exposed, so nothing can write a
//! value without its metadata row, and the `*_with_metadata` endpoints below
//! update both stores in a single call.
use candid::{CandidType, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Blob;
use ic_stable_structures::{storable::Bound, Storable};
use ic_stable_structures::{BTreeMap as StableBTreeMap, DefaultMemoryImpl};
use ic_vetkeys::types::{ByteBuf, EncryptedMapValue};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PasswordMetadata {
    creation_date: u64,
    last_modification_date: u64,
    number_of_modifications: u64,
    last_modified_principal: Principal,
    tags: Vec<String>,
    url: String,
}

impl PasswordMetadata {
    pub fn new(caller: Principal, tags: Vec<String>, url: String) -> Self {
        let time_now = ic_cdk::api::time();
        Self {
            creation_date: time_now,
            last_modification_date: time_now,
            number_of_modifications: 0,
            last_modified_principal: caller,
            tags,
            url,
        }
    }

    pub fn update(self, caller: Principal, tags: Vec<String>, url: String) -> Self {
        let time_now = ic_cdk::api::time();
        Self {
            creation_date: self.creation_date,
            last_modification_date: time_now,
            number_of_modifications: self.number_of_modifications + 1,
            last_modified_principal: caller,
            tags,
            url,
        }
    }
}

impl Storable for PasswordMetadata {
    fn into_bytes(self) -> Vec<u8> {
        self.to_bytes().into_owned()
    }

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(serde_cbor::to_vec(self).expect("failed to serialize"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(bytes.as_ref()).expect("failed to deserialize")
    }

    const BOUND: Bound = Bound::Unbounded;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
type MapOwner = Principal;
type MapName = Blob<32>;
type MapKey = Blob<32>;
// To understand the intuition how a stable map over a tuple type works, see
// https://mmapped.blog/posts/14-stable-structures#stable-btree.
type StableMetadataMap = StableBTreeMap<(MapOwner, MapName, MapKey), PasswordMetadata, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    // Use `init` (not `new`): after an upgrade the thread-local is re-created,
    // and `new` would overwrite the existing map with an empty one, dropping all
    // metadata. `init` loads the persisted map when the memory already holds one.
    static METADATA: RefCell<StableMetadataMap> = RefCell::new(StableBTreeMap::init(memory(4)));
}

fn memory(id: u8) -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(id)))
}

// The first argument is the domain-separator string that isolates this
// application's derived keys; it must stay stable for the life of the canister.
// The four Memory instances back EncryptedMaps' own stable state, in the order
// it expects: config (which persists the domain separator and vetKD key id),
// access control, shared keys, and the encrypted values. This canister's own
// metadata map lives in the same MemoryManager under id 4.
//
// The generated `#[init]` takes the vetKD key name (see `init_args` in
// `icp.yaml`); `#[post_upgrade]` re-reads it from the persisted config.
ic_vetkeys::export_encrypted_maps_canister!(
    "password_manager_with_metadata_app",
    [memory(0), memory(1), memory(2), memory(3)],
    custom_value_endpoints,
);

#[ic_cdk::query]
fn get_encrypted_values_for_map_with_metadata(
    map_owner: Principal,
    map_name: ByteBuf,
) -> Result<Vec<(ByteBuf, EncryptedMapValue, PasswordMetadata)>, String> {
    let map_name = bytebuf_to_blob(map_name)?;
    let map_id = (map_owner, map_name);
    let map_values = with_encrypted_maps(|encrypted_maps| {
        encrypted_maps.get_encrypted_values_for_map(ic_cdk::api::msg_caller(), map_id)
    })?;

    // Look the metadata up per key rather than zipping two ordered iterators:
    // the pairing then cannot silently shift if the two stores ever disagree.
    // A missing row would mean the invariant maintained by the insert/remove
    // endpoints below was broken, so surface it instead of hiding the entry.
    METADATA.with_borrow(|metadata| {
        map_values
            .into_iter()
            .map(|(map_key, encrypted_value)| {
                let password_metadata = metadata
                    .get(&(map_owner, map_name, map_key))
                    .ok_or_else(|| "missing metadata for stored password".to_string())?;
                Ok((
                    ByteBuf::from(map_key.as_slice().to_vec()),
                    encrypted_value,
                    password_metadata,
                ))
            })
            .collect()
    })
}

#[ic_cdk::update]
fn insert_encrypted_value_with_metadata(
    map_owner: Principal,
    map_name: ByteBuf,
    map_key: ByteBuf,
    value: EncryptedMapValue,
    tags: Vec<String>,
    url: String,
) -> Result<Option<(EncryptedMapValue, PasswordMetadata)>, String> {
    let caller = ic_cdk::api::msg_caller();
    let map_name = bytebuf_to_blob(map_name)?;
    let map_id = (map_owner, map_name);
    let map_key = bytebuf_to_blob(map_key)?;
    // The library call performs the access-control check, so it comes first:
    // if the caller may not write, the metadata store is left untouched.
    let opt_prev_value = with_encrypted_maps_mut(|encrypted_maps| {
        encrypted_maps.insert_encrypted_value(caller, map_id, map_key, value)
    })?;
    Ok(METADATA.with_borrow_mut(|metadata| {
        let metadata_key = (map_owner, map_name, map_key);
        let metadata_value = metadata
            .get(&metadata_key)
            .map(|m| m.update(caller, tags.clone(), url.clone()))
            .unwrap_or(PasswordMetadata::new(caller, tags, url));
        opt_prev_value.zip(metadata.insert(metadata_key, metadata_value))
    }))
}

#[ic_cdk::update]
fn remove_encrypted_value_with_metadata(
    map_owner: Principal,
    map_name: ByteBuf,
    map_key: ByteBuf,
) -> Result<Option<(EncryptedMapValue, PasswordMetadata)>, String> {
    let map_name = bytebuf_to_blob(map_name)?;
    let map_id = (map_owner, map_name);
    let map_key = bytebuf_to_blob(map_key)?;
    let opt_prev_value = with_encrypted_maps_mut(|encrypted_maps| {
        encrypted_maps.remove_encrypted_value(ic_cdk::api::msg_caller(), map_id, map_key)
    })?;
    Ok(METADATA.with_borrow_mut(|metadata| {
        opt_prev_value.zip(metadata.remove(&(map_owner, map_name, map_key)))
    }))
}

fn bytebuf_to_blob(buf: ByteBuf) -> Result<Blob<32>, String> {
    Blob::try_from(buf.as_ref()).map_err(|_| "too large input".to_string())
}

ic_cdk::export_candid!();
