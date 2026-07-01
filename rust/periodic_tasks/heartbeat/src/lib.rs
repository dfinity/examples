//! An example of periodic tasks scheduling using heartbeats.

use std::{
    cell::RefCell,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime},
};

thread_local! {
    /// The global counter to increment periodically.
    static COUNTER: RefCell<u32> = RefCell::new(0);
    /// Last time the heartbeat was called at. It is implementation-defined
    /// when exactly the heartbeat handler will be called, so some manual
    /// time tracking is required to make sure the interval has passed.
    static LAST_PERIODIC_TASK_TIME: RefCell<SystemTime> = RefCell::new(std::time::UNIX_EPOCH);
}

/// Default interval is at least 10 seconds.
static MIN_INTERVAL_SECS: AtomicU64 = AtomicU64::new(10);
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
        ic_cdk::println!("Heartbeat canister: Counter: {}", counter.borrow());
    });

    track_cycles_used();
}

/// Tracks the amount of cycles used for the periodic task.
fn track_cycles_used() {
    // Update the `INITIAL_CANISTER_BALANCE` if needed.
    // Cast u128 → u64: safe in practice as canister cycle balances fit within u64.
    let current_canister_balance = ic_cdk::api::canister_cycle_balance() as u64;
    INITIAL_CANISTER_BALANCE.fetch_max(current_canister_balance, Ordering::Relaxed);
    // Store the difference between the initial and the current balance.
    let cycles_used = INITIAL_CANISTER_BALANCE.load(Ordering::Relaxed) - current_canister_balance;
    CYCLES_USED.store(cycles_used, Ordering::Relaxed);
}

/// This is special `canister_heartbeat` method which is invoked by
/// the Internet Computer periodically.
///
/// The only way to disable the heartbeats is to upgrade the canister
/// to a version which does not export the `canister_heartbeat` method.
/// Also, the heartbeat interval is implementation-defined, and there is
/// no way to adjust it.
///
/// The logic to schedule periodic tasks at the specific intervals must be
/// implemented manually inside the heartbeat method. Below is a simplistic
/// example implementation.
#[ic_cdk::heartbeat]
fn heartbeat() {
    let time_nanos = ic_cdk::api::time();
    let now = SystemTime::UNIX_EPOCH + Duration::from_nanos(time_nanos);
    LAST_PERIODIC_TASK_TIME.with(|last_periodic_task_time| {
        let min_interval_secs = MIN_INTERVAL_SECS.load(Ordering::Relaxed);
        // Check if it's time to call the periodic task.
        if *last_periodic_task_time.borrow() + Duration::from_secs(min_interval_secs) < now {
            // Note, the heartbeat code and the periodic task are executed
            // in the same context. If the periodic task fails, all the
            // changes will be reverted by the IC, i.e. the `LAST_PERIODIC_TASK_TIME`
            // variable won't be updated.
            //
            // To isolate the execution contexts of the scheduling logic and
            // the periodic task, spawn a self canister call instead of calling
            // the function directly (heartbeat is sync, so use spawn):
            //   ic_cdk::spawn(async {
            //     ic_cdk::call::Call::bounded_wait(ic_cdk::id(), "periodic_task").await.ok();
            //   });
            periodic_task();
            // Update the time when the periodic task was last called.
            *last_periodic_task_time.borrow_mut() = now;
        }
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

/// Sets a periodic tasks interval in seconds to increment the `COUNTER`.
#[ic_cdk::update]
fn set_interval_secs(secs: u64) {
    MIN_INTERVAL_SECS.store(secs, Ordering::Relaxed);
    ic_cdk::println!("Heartbeat canister: Setting interval to {secs}s...");

    // To drive an async function to completion inside the heartbeat handler,
    // use `ic_cdk::spawn()`, for example:
    // ic_cdk::spawn(async_function());
}

/// Stops incrementing the counter by setting a huge interval.
#[ic_cdk::update]
fn stop() {
    // Due to the huge interval the periodic task won't be called.
    set_interval_secs(1_000_000_000_000);
}

/// Returns the amount of cycles used since the beginning of the execution.
#[ic_cdk::query]
fn cycles_used() -> u64 {
    CYCLES_USED.load(Ordering::Relaxed)
}

////////////////////////////////////////////////////////////////////////
// Handling canister initialization and upgrades
////////////////////////////////////////////////////////////////////////

// For the heartbeats to work, the canister should just export the `canister_heartbeat`
// method, no initialization is required. However, the developer is responsible
// to serialize settings (like `LAST_PERIODIC_TASK_TIME` and `MIN_INTERVAL_SECS`) in
// `canister_pre_upgrade` method and/or re-initialize them in the `canister_post_upgrade`.

/// This is special `canister_init` method which is invoked by
/// the Internet Computer when the canister is installed for the first time.
#[ic_cdk::init]
fn init(min_interval_secs: u64) {
    set_interval_secs(min_interval_secs);
}

/// This is special `canister_post_upgrade` method which is invoked by
/// the Internet Computer after the canister is upgraded to a new version.
#[ic_cdk::post_upgrade]
fn post_upgrade(min_interval_secs: u64) {
    init(min_interval_secs);
}

ic_cdk::export_candid!();
