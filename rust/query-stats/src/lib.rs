use candid::{CandidType, Nat};
use ic_cdk::api::management_canister::main::canister_status;
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;

use ic_cdk::api::management_canister::main::QueryStats;

use std::cell::RefCell;
use std::time::Duration;

const TIMER_PERIOD_IN_SECS: u64 = 5;
const QUERY_STATS_PURGE_AFTER_SECS: u64 = 60 * 60 * 24; // 24h, has to be much larger than query stats epoch
const QUERY_STATS_RATE_FOR_LAST_SECS: u64 = 60 * 60; // 1h, has to be larger than query stats epoch

// Unfortunately, we need to redefine this type, since the members are not public.
#[derive(CandidType)]
pub struct QueryStatRates {
    num_calls_total: f32,
    num_instructions_total: f32,
    request_payload_bytes_total: f32,
    response_payload_bytes_total: f32,
}

struct QueryStatAtTime {
    timestamp_nanos: u64,
    query_stats: QueryStats,
}

impl QueryStatAtTime {
    fn rate(&self, other: &Self) -> QueryStatRates {
        // Make sure we are subtracting the older from the newer.
        let time_diff = self.timestamp_nanos - other.timestamp_nanos;
        assert!(time_diff > 0);

        fn calc_rate(a: &Nat, b: &Nat, time_diff: u64) -> f32 {
            let a: u128 = a.0.clone().try_into().unwrap();
            let b: u128 = b.0.clone().try_into().unwrap();
            let time_diff = time_diff as f32;
            (a as f32 - b as f32) / time_diff
        }

        QueryStatRates {
            num_calls_total: calc_rate(
                &self.query_stats.num_calls_total,
                &other.query_stats.num_calls_total,
                time_diff,
            ),
            num_instructions_total: calc_rate(
                &self.query_stats.num_instructions_total,
                &other.query_stats.num_instructions_total,
                time_diff,
            ),
            request_payload_bytes_total: calc_rate(
                &self.query_stats.request_payload_bytes_total,
                &other.query_stats.request_payload_bytes_total,
                time_diff,
            ),
            response_payload_bytes_total: calc_rate(
                &self.query_stats.response_payload_bytes_total,
                &other.query_stats.response_payload_bytes_total,
                time_diff,
            ),
        }
    }
}

#[derive(Default)]
struct QueryStatsRates {
    query_stats: Vec<QueryStatAtTime>,
}

thread_local! {
    /// Maintain a list of data points for query stats on the heap.
    ///
    /// Will currently not be preserved during canister upgrades.
    static QUERY_STATS_RATE: RefCell<QueryStatsRates> = RefCell::new(QueryStatsRates::default());
}

/// A helper method to mutate the state.
pub(crate) fn with_state_mut<R>(f: impl FnOnce(&mut QueryStatsRates) -> R) -> R {
    QUERY_STATS_RATE.with(|cell| f(&mut cell.borrow_mut()))
}

/// A helper method to access the state.
pub(crate) fn with_state<R>(f: impl FnOnce(&QueryStatsRates) -> R) -> R {
    QUERY_STATS_RATE.with(|cell| f(&cell.borrow()))
}

#[ic_cdk::query]
async fn get_current_query_stats_as_string() -> String {
    let c = canister_status(CanisterIdRecord {
        canister_id: ic_cdk::id(),
    })
    .await;
    let query_stats = c.map(|c| c.0.query_stats);
    format!("{:?}", query_stats)
}

#[ic_cdk::query]
// Calculates the rate between the latest query statistics and the query statistics
// from QUERY_STATS_RATE_FOR_LAST_SECS seconds ago.
fn get_current_query_stats_as_rates() -> Option<QueryStatRates> {
    with_state(|state| {
        let mut most_recent: Option<&QueryStatAtTime> = None;
        let mut before: Option<&QueryStatAtTime> = None;

        for q in &state.query_stats {
            // Find most recent query stats data point
            if most_recent.is_none() || q.timestamp_nanos > most_recent.unwrap().timestamp_nanos {
                most_recent = Some(q);
            }

            // Find query stats data point at least QUERY_STATS_RATE_FOR_LAST_SECS old
            if (q.timestamp_nanos
                + Duration::from_secs(QUERY_STATS_RATE_FOR_LAST_SECS).as_nanos() as u64)
                < ic_cdk::api::time()
            {
                if before.is_none() || q.timestamp_nanos > before.unwrap().timestamp_nanos {
                    before = Some(q);
                }
            }
        }

        if let Some(most_recent) = most_recent {
            if let Some(before) = before {
                Some(most_recent.rate(before))
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// Periodic task to fetch query stats and store them on the heap.
async fn periodic_task() {
    // Fetch current query stats.
    let canister_status = canister_status(CanisterIdRecord {
        canister_id: ic_cdk::id(),
    })
    .await;

    if let Ok(canister_status) = canister_status {
        let timestamp_nanos = ic_cdk::api::time();
        with_state_mut(|state| {
            // Store the current query stats.
            state.query_stats.push(QueryStatAtTime {
                timestamp_nanos: timestamp_nanos,
                query_stats: canister_status.0.query_stats,
            });

            // Remove the old query stats.
            let purge_at = timestamp_nanos
                - Duration::from_secs(QUERY_STATS_PURGE_AFTER_SECS).as_nanos() as u64;
            state.query_stats.retain(|q| q.timestamp_nanos > purge_at);
        });
    }
}

////////////////////////////////////////////////////////////////////////
// Setup and maintain timers for periodically retrieving query stats
////////////////////////////////////////////////////////////////////////

/// Starts a new periodic tasks to retrieve the query stats.
fn start_timer() {
    let secs = Duration::from_secs(TIMER_PERIOD_IN_SECS);
    ic_cdk::println!("Query stats: Starting a new timer with {secs:?} interval...");
    // Schedule a new periodic task to fetch query stats.
    ic_cdk_timers::set_timer_interval(secs, || ic_cdk::spawn(periodic_task()));
}

/// This is special `canister_init` method which is invoked by
/// the Internet Computer when the canister is installed for the first time.
#[ic_cdk_macros::init]
fn init() {
    start_timer();
}

/// This is special `canister_post_upgrade` method which is invoked by
/// the Internet Computer after the canister is upgraded to a new version.
///
/// Note, after the canister is upgraded, all the timers get deactivated.
/// The developer is responsible to track and serialize the timers into
/// the stable memory in `canister_pre_upgrade` method and/or re-activate
/// the timers in the `canister_post_upgrade`.
#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    start_timer();
}
