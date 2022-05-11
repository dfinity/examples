use std::cell::RefCell;

use ic_cdk::export::candid;
use ic_cdk_macros::*;

thread_local! {
    static COUNTER: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0));
}

#[update]
fn increment() -> candid::Nat {
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1u64;
    });

    COUNTER.with(|counter| counter.borrow().clone())
}

#[query]
fn get() -> candid::Nat {
    COUNTER.with(|counter| counter.borrow().clone())
}

#[update]
fn set(input: candid::Nat) -> () {
    COUNTER.with(|counter| {
        *counter.borrow_mut() = input;
    })
}
