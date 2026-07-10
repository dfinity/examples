use candid::types::number::Nat;
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

/// Get the value of the counter.
#[ic_cdk::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Set the value of the counter.
#[ic_cdk::update]
fn set(n: Nat) {
    COUNTER.with(|count| *count.borrow_mut() = n);
}

#[ic_cdk::update]
fn get_and_set(n: Nat) -> Nat {
    COUNTER.with(|counter| {
        let old = counter.borrow().clone();
        *counter.borrow_mut() = n;
        old
    })
}

/// Increment the value of the counter.
#[ic_cdk::update]
fn increment() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1_u32);
}

ic_cdk::export_candid!();
