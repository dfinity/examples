use candid::types::number::Nat;
use ic_cdk::timer;
use std::{cell::RefCell, time::Duration};

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

/// Get the value of the counter.
#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Initialize a timer.
#[ic_cdk_macros::init]
fn init() {
    // Increment the value of the `COUNTER` every ~10 seconds.
    let _id = timer::set_timer_interval(Duration::from_secs(10), || {
        COUNTER.with(|counter| *counter.borrow_mut() += 1)
    });
}

/// All canister timers are deactivated after an upgrade.
#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert_eq!(get(), Nat::from(0));
    }
}
