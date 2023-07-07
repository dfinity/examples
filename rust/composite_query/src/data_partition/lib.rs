use ic_cdk_macros::{query, update};
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use std::cell::RefCell;

thread_local! {
    static STORE: RefCell<BTreeMap<u128, u128, DefaultMemoryImpl>> = RefCell::new(
        BTreeMap::init(DefaultMemoryImpl::default()));
}

#[update]
fn put(key: u128, value: u128) -> Option<u128> {
    ic_cdk::println!("Set in backend for key={} with value={}", key, value);
    STORE.with(|store| store.borrow_mut().insert(key, value))
}

#[query]
fn get(key: u128) -> Option<u128> {
    STORE.with(|store| {
        let r = store.borrow().get(&key);
        ic_cdk::println!("Get in backend for key={} - result={:?}", key, r);
        r
    })
}
