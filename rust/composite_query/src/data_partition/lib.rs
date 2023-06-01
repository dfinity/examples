use ic_cdk_macros::{query, update};
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use std::cell::RefCell;

thread_local! {
    static STORE: RefCell<BTreeMap<u128, u128, DefaultMemoryImpl>> = RefCell::new(
        BTreeMap::init(DefaultMemoryImpl::default()));
}

#[update]
fn insert(key: u128, value: u128) -> Option<u128> {
    STORE.with(|store| store.borrow_mut().insert(key, value))
}

#[query]
fn lookup(key: u128) -> Option<u128> {
    STORE.with(|store| store.borrow().get(&key))
}
