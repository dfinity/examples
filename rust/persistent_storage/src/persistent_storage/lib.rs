use std::cell::RefCell;
use serde::{Deserialize};
use candid::{CandidType};
use ic_cdk::{
    export::candid,
    storage,
};
use ic_cdk_macros::*;

thread_local! {
    static COUNTER: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0));
}

#[derive(CandidType, Deserialize)]
struct StableState {
    count: candid::Nat,
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

#[pre_upgrade]
fn pre_upgrade() {
    let count = COUNTER.with(|counter| counter.borrow().clone());
    let stable_state = StableState { count };
    storage::stable_save((stable_state,)).unwrap();
}
#[post_upgrade]
fn post_upgrade() {
    let (StableState { count },) = storage::stable_restore().unwrap();
    COUNTER.with(|counter| {
        *counter.borrow_mut() = count;
    })
}
