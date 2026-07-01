//! An example of periodic tasks scheduling using timers.

use ic_cdk_timers::TimerId;
use std::{cell::RefCell, time::Duration};

thread_local! {
    /// The global counter to increment periodically.
    static COUNTER: RefCell<u32> = RefCell::new(0);
    /// The global vector to keep multiple timer IDs.
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
    /// Peak canister balance observed so far — used as baseline for tracking cycles spent.
    /// Resets upward if the canister receives additional cycles (top-up).
    static PEAK_CANISTER_BALANCE: RefCell<u128> = RefCell::new(0);
    /// Canister cycles usage tracked in the periodic task.
    static CYCLES_USED: RefCell<u128> = RefCell::new(0);
}

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
    let current = ic_cdk::api::canister_cycle_balance();
    // Update `PEAK_CANISTER_BALANCE` to the highest observed balance.
    PEAK_CANISTER_BALANCE.with(|initial| {
        if current > *initial.borrow() {
            *initial.borrow_mut() = current;
        }
    });
    // Store the difference between the peak and the current balance.
    CYCLES_USED.with(|used| {
        *used.borrow_mut() =
            PEAK_CANISTER_BALANCE.with(|peak| *peak.borrow()) - current;
    });
}

////////////////////////////////////////////////////////////////////////
// Canister interface
////////////////////////////////////////////////////////////////////////

/// Returns the `COUNTER` value.
#[ic_cdk::query]
fn counter() -> u32 {
    COUNTER.with(|counter| *counter.borrow())
}

/// Starts a new periodic task to increment the `COUNTER` with specified
/// interval in seconds.
///
/// It is implementation-defined when exactly the timer handler will be called.
/// The only explicit guarantee is that it won't be called earlier.
#[ic_cdk::update]
fn start_with_interval_secs(secs: u64) {
    let secs = Duration::from_secs(secs);
    ic_cdk::println!("Timer canister: Starting a new timer with {secs:?} interval...");
    // Schedule a new periodic task to increment the counter.
    // ic-cdk-timers 1.0 requires the closure to return a Future.
    let timer_id = ic_cdk_timers::set_timer_interval(secs, || async { periodic_task() });
    // Add the timer ID to the global vector.
    TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));

    // To call an async function from a timer handler, use it directly:
    // ic_cdk_timers::set_timer_interval(interval, || async { async_function().await });
}

/// Stops incrementing the counter by clearing the last timer ID.
#[ic_cdk::update]
fn stop() {
    TIMER_IDS.with(|timer_ids| {
        if let Some(timer_id) = timer_ids.borrow_mut().pop() {
            ic_cdk::println!("Timer canister: Stopping timer ID {timer_id:?}...");
            // It's safe to clear non-existent timer IDs.
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

/// Returns the cycles spent since the peak canister balance was last observed.
/// Resets if the canister is topped up (a top-up raises the peak, resetting the delta).
#[ic_cdk::query]
fn cycles_used() -> u128 {
    CYCLES_USED.with(|used| *used.borrow())
}

////////////////////////////////////////////////////////////////////////
// Handling canister initialization and upgrades
////////////////////////////////////////////////////////////////////////

/// This is special `canister_init` method which is invoked by
/// the Internet Computer when the canister is installed for the first time.
#[ic_cdk::init]
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
#[ic_cdk::post_upgrade]
fn post_upgrade(min_interval_secs: u64) {
    init(min_interval_secs);
}

ic_cdk::export_candid!();
