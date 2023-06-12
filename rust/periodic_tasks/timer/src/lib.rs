//! An example of periodic tasks scheduling using timers.

use ic_cdk_timers::TimerId;
use std::{
    cell::RefCell,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

thread_local! {
    /// The global counter to increment periodically.
    static COUNTER: RefCell<u32> = RefCell::new(0);
    /// The global vector to keep multiple timer IDs.
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}

/// Initial canister balance to track the cycles usage.
static INITIAL_CANISTER_BALANCE: AtomicU64 = AtomicU64::new(0);
/// Canister cycles usage tracked in the periodic task.
static CYCLES_USED: AtomicU64 = AtomicU64::new(0);

////////////////////////////////////////////////////////////////////////
// Periodic task
////////////////////////////////////////////////////////////////////////

/// An example periodic task which just increments the `COUNTER`.
fn periodic_task() {
    // Just increment the counter.
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        ic_cdk::println!("Timer canister: Counter: {}", counter.borrow());
    });

    track_cycles_used();
}

/// Tracks the amount of cycles used for the periodic task.
fn track_cycles_used() {
    // Update the `INITIAL_CANISTER_BALANCE` if needed.
    let current_canister_balance = ic_cdk::api::canister_balance();
    INITIAL_CANISTER_BALANCE.fetch_max(current_canister_balance, Ordering::Relaxed);
    // Store the difference between the initial and the current balance.
    let cycles_used = INITIAL_CANISTER_BALANCE.load(Ordering::Relaxed) - current_canister_balance;
    CYCLES_USED.store(cycles_used, Ordering::Relaxed);
}

////////////////////////////////////////////////////////////////////////
// Canister interface
////////////////////////////////////////////////////////////////////////

/// Returns the `COUNTER` value.
///
/// Example usage: `dfx canister call timer counter`
#[ic_cdk_macros::query]
fn counter() -> u32 {
    COUNTER.with(|counter| *counter.borrow())
}

/// Starts a new periodic tasks to increment the `COUNTER` with specified
/// interval in seconds.
///
/// It is implementation-defined when exactly the timer handler will be called.
/// The only explicit guarantee is that it won't be called earlier.
///
/// Example usage: `dfx canister call timer start_with_interval_secs 10`
#[ic_cdk_macros::update]
fn start_with_interval_secs(secs: u64) {
    let secs = Duration::from_secs(secs);
    ic_cdk::println!("Timer canister: Starting a new timer with {secs:?} interval...");
    // Schedule a new periodic task to increment the counter.
    let timer_id = ic_cdk_timers::set_timer_interval(secs, periodic_task);
    // Add the timer ID to the global vector.
    TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));

    // To drive an async function to completion inside the timer handler,
    // use `ic_cdk::spawn()`, for example:
    // ic_cdk_timers::set_timer_interval(interval, || ic_cdk::spawn(async_function()));
}

/// Stops incrementing the counter by clearing the last timer ID.
///
/// Example usage: `dfx canister call timer stop`
#[ic_cdk_macros::update]
fn stop() {
    TIMER_IDS.with(|timer_ids| {
        if let Some(timer_id) = timer_ids.borrow_mut().pop() {
            ic_cdk::println!("Timer canister: Stopping timer ID {timer_id:?}...");
            // It's safe to clear non-existent timer IDs.
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

/// Returns the amount of cycles used since the beginning of the execution.
///
/// Example usage: `dfx canister call timer cycles_used`
#[ic_cdk_macros::query]
fn cycles_used() -> u64 {
    CYCLES_USED.load(Ordering::Relaxed)
}

////////////////////////////////////////////////////////////////////////
// Handling canister initialization and upgrades
////////////////////////////////////////////////////////////////////////

/// This is special `canister_init` method which is invoked by
/// the Internet Computer when the canister is installed for the first time.
#[ic_cdk_macros::init]
fn init(min_interval_secs: u64) {
    start_with_interval_secs(min_interval_secs);
}

/// This is special `canister_post_upgrade` method which is invoked by
/// the Internet Computer after the canister is upgraded to a new version.
///
/// Note, after the canister is upgraded, all the timers get deactivated.
/// The developer is responsible to track and serialize the timers into
/// the stable memory in `canister_pre_upgrade` method and/or re-activate
/// the timers in the `canister_post_upgrade`.
#[ic_cdk_macros::post_upgrade]
fn post_upgrade(min_interval_secs: u64) {
    init(min_interval_secs);
}
