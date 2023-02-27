use exchange_rate::{Rate, RatesWithInterval, TimeRange, Timestamp};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_cdk::storage;
use ic_cdk_macros::{self, heartbeat, post_upgrade, pre_upgrade, query, update};
use serde_json::{self, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

// How many data point can be returned as maximum.
// Given that 2MB is max-allow canister response size, and each <Timestamp, Rate> pair
// should be less that 20 bytes. Maximum data points could be returned for each
// call can be as many as 2MB / 20B = 100000.
pub const MAX_DATA_PONTS_CANISTER_RESPONSE: usize = 100000;

// Remote fetch interval in secs. It is only the canister returned interval
// that is dynamic according to the data size needs to be returned.
pub const REMOTE_FETCH_GRANULARITY: u64 = 60;

/// The period we wait before issuing a new external HTTPS request.
pub const TRY_FETCH_HEARTBEAT_PERIOD: usize = 5;

// How many data points in each Coinbase API call. Maximum allowed is 300
pub const DATA_POINTS_PER_API: u64 = 200;

// Maximum raw Coinbase API response size. This field is used by IC to calculate circles cost per HTTP call.
// Here is how this number is derived:
// Each Coinbase API call return an array of array, and each sub-array look like below:
// [
//     1652454000,
//     9.51,
//     9.6,
//     9.55,
//     9.54,
//     4857.1892
// ],
// Each field of this sub-arry takes less than 10 bytes. Then,
// 10 (bytes per field) * 6 (fields per timestamp) * 200 (timestamps)
pub const MAX_RESPONSE_BYTES: u64 = 10 * 6 * DATA_POINTS_PER_API;

thread_local! {
    pub static FETCHED: RefCell<HashMap<Timestamp, Rate>>  = RefCell::new(HashMap::new());
    pub static REQUESTED: RefCell<HashSet<Timestamp>> = RefCell::new(HashSet::new());
    pub static RATE_COUNTER: RefCell<usize> = RefCell::new(0);
}

/// On every 'TRY_FETCH_HEARTBEAT_PERIOD' heartbeat, issuing a new HTTPS outcall.
#[heartbeat]
async fn heartbeat() {
    let mut should_fetch = false;
    RATE_COUNTER.with(|counter| {
        let mut count = counter.borrow_mut();
        if *count == 0 {
            should_fetch = true;
        }
        *count = (*count + 1) % TRY_FETCH_HEARTBEAT_PERIOD;
    });
    if should_fetch {
        get_next_rate().await;
    }
}

/*
Get rates for a time range defined by start time and end time. This function can be invoked
as HTTP update call.
*/
#[update]
async fn get_rates(range: TimeRange) -> RatesWithInterval {
    // round down start time and end time to the minute (chop off seconds), to be checked in the hashmap
    let start_min = range.start / REMOTE_FETCH_GRANULARITY;
    let end_min = range.end / REMOTE_FETCH_GRANULARITY;

    // compose a return structure
    let mut fetched = HashMap::new();

    // pull available ranges from hashmap
    FETCHED.with(|map| {
        let map = map.borrow();
        for requested_min in start_min..end_min {
            let requested = requested_min * REMOTE_FETCH_GRANULARITY;
            if map.contains_key(&requested) {
                // The fetched slot is within user requested range. Add to result for later returning.
                ic_cdk::api::print(format!("Found {} in map!", requested));
                fetched.insert(requested, map.get(&requested).unwrap().clone());
            } else {
                ic_cdk::api::print(format!("Did not find {} in map!", requested));
                // asynchoronously request downloads for unavailable ranges

                add_job_to_job_set(requested);
            }
        }
    });

    // return sampled rates for available ranges
    sample_with_interval(fetched)
}

fn sample_with_interval(fetched: HashMap<Timestamp, Rate>) -> RatesWithInterval {
    // in order to make sure that returned data do not exceed 2MB, which is about
    // ~1M data points, calculate interval when data points count is beyond 900K.
    let interval_options = vec![
        1,       // 1 data point every minute
        5,       // 1 data point every 5 minutes
        15,      // 1 data point every 15 minutes
        60,      // 1 data point every hour
        60 * 12, // 1 data point every 12 hours
        60 * 24, // 1 data point every day
    ];
    for i in interval_options {
        if fetched.len() / i < MAX_DATA_PONTS_CANISTER_RESPONSE {
            return RatesWithInterval {
                interval: i * REMOTE_FETCH_GRANULARITY as usize,
                rates: fetched
                    .into_iter()
                    .filter(|r| r.0 % (i as u64) == 0)
                    .collect(),
            };
        }
    }
    panic!("This shouldn't be happening! Couldn't find an inteval that can keep total data points count in {}.", MAX_DATA_PONTS_CANISTER_RESPONSE);
}

fn add_job_to_job_set(job: Timestamp) -> () {
    // Since Coinbase API allows DATA_POINTS_PER_API data points (5 hours of data) per API call,
    // and the response size is roughly 14KB, which is way below max_response_size,
    // we normalize the job to the beginning of 5 hours.
    REQUESTED.with(|set| {
        let mut set = set.borrow_mut();
        let normalized_job = job / (REMOTE_FETCH_GRANULARITY * DATA_POINTS_PER_API)
            * (REMOTE_FETCH_GRANULARITY * DATA_POINTS_PER_API);
        set.insert(normalized_job);
    });
}

/// Triggered by the 'heartbeat()' function, picks up the next job in the pipe for
/// executing a remote HTTPS outcall.
async fn get_next_rate() {
    let mut job_id: u64 = 0;

    // Get the next downloading job
    REQUESTED.with(|set| {
        let mut set = set.borrow_mut();

        if set.len() == 0 {
            ic_cdk::api::print("Request set is empty, no more jobs to fetch.");
            return;
        }

        let item_to_remove = set.iter().next().cloned().unwrap();
        if !set.remove(&item_to_remove) {
            ic_cdk::api::print("Item not found in job set.");
            return;
        }

        // Job is a valid
        job_id = item_to_remove;

        FETCHED.with(|fetched| {
            match fetched.borrow().get(&item_to_remove) {
                Some(_) => {
                    // If this job has already been downloaded. Only downloading it if doesn't already exist.
                    ic_cdk::api::print(format!(
                        "Rate for {} is already downloaded. Skipping downloading again.",
                        item_to_remove
                    ));
                    return;
                }
                None => {
                    // The requested time rate isn't found in map. Send a canister get_rate call to self
                    ic_cdk::api::print(format!("Fetching job {} now.", item_to_remove));
                }
            }
        });
    });
    if job_id != 0 {
        get_rate(job_id).await;
    }
}

/// Calls the IC 'http_request' function with sample interval of REMOTE_FETCH_GRANULARITY seconds.
/// Each API fetches DATA_POINTS_PER_API data points, which is equivalent of DATA_POINTS_PER_API minutes of data.
async fn get_rate(job: Timestamp) {
    let start_timestamp = job;
    let end_timestamp = job + REMOTE_FETCH_GRANULARITY * DATA_POINTS_PER_API;

    let host = "api.pro.coinbase.com";
    let mut host_header = host.clone().to_owned();
    host_header.push_str(":443");
    // prepare system http_request call
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: host_header,
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];
    let url = format!("https://{host}/products/ICP-USD/candles?granularity={REMOTE_FETCH_GRANULARITY}&start={start_timestamp}&end={end_timestamp}");
    ic_cdk::api::print(url.clone());

    ic_cdk::api::print(format!("Making IC http_request call {} now.", job));
    let request = CanisterHttpRequestArgument {
        url: url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform: Some(TransformContext::new(transform, vec![])),
        headers: request_headers,
    };
    match http_request(request).await {
        Ok((response,)) => {
            // put the result to hashmap
            FETCHED.with(|fetched| {
                let mut fetched = fetched.borrow_mut();
                let str_body = String::from_utf8(response.body)
                    .expect("Transformed response is not UTF-8 encoded.");
                for bucket in str_body.lines() {
                    let mut iter = bucket.split_whitespace();
                    let ts = iter.next().unwrap().parse::<Timestamp>().unwrap();
                    let rate = iter.next().unwrap().parse::<Rate>().unwrap();
                    assert!(iter.next().is_none());
                    fetched.insert(ts, rate);
                }
            });
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            ic_cdk::api::print(message.clone());

            // Since the remote request failed. Adding the de-queued job back again for retries.
            add_job_to_job_set(job);
        }
    }
}

fn keep_bucket_start_time_and_closing_price(body: &[u8]) -> Vec<u8> {
    //ic_cdk::api::print(format!("Got decoded result: {}", body));
    let rates_array: Vec<Vec<Value>> = serde_json::from_slice(body).unwrap();
    let mut res = vec![];
    for rate in rates_array {
        let bucket_start_time = rate[0].as_u64().expect("Couldn't parse the time.");
        let closing_price = rate[4].as_f64().expect("Couldn't parse the rate.");
        res.append(
            &mut format!("{} {}\n", bucket_start_time, closing_price)
                .as_bytes()
                .to_vec(),
        );
    }
    res
}

/// Strips all data that is not needed from the original response.
#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        ..Default::default()
    };
    if res.status == 200 {
        res.body = keep_bucket_start_time_and_closing_price(&raw.response.body)
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    res
}

#[pre_upgrade]
fn pre_upgrade() {
    FETCHED.with(|fetched| storage::stable_save((fetched,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (old_fetched,): (HashMap<Timestamp, Rate>,) = storage::stable_restore().unwrap();
    FETCHED.with(|fetched| *fetched.borrow_mut() = old_fetched);
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_body_to_rates() {
        let body = "
[
    [
        1652454300,
        9.51,
        9.59,
        9.54,
        9.51,
        14184.2377
    ],
    [
        1652454240,
        9.51,
        9.55,
        9.55,
        9.52,
        2385.9735
    ],
    [
        1652454180,
        9.54,
        9.58,
        9.55,
        9.56,
        1930.129
    ]
]
        ";
        let res = keep_bucket_start_time_and_closing_price(body.as_bytes());
        let expected = "1652454300 9.51\n1652454240 9.52\n1652454180 9.56\n";
        assert_eq!(res, expected.as_bytes().to_vec());
    }
}
