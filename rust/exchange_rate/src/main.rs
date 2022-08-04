use candid::{CandidType, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{self, heartbeat, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};

type Timestamp = u64;
type Rate = f32;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Clone, Debug, PartialEq, CandidType, Serialize, Deserialize)]
pub struct RatesWithInterval {
    pub interval: usize,
    pub rates: HashMap<Timestamp, Rate>,
}

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, CandidType, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterHttpRequestArgs {
    pub url: String,
    pub max_response_bytes: Option<u64>,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
    pub http_method: HttpMethod,
    pub transform_method_name: Option<String>,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanisterHttpResponsePayload {
    pub status: u64,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

// How many data point can be returned as maximum.
// Given that 2MB is max-allow canister response size, and each <Timestamp, Rate> pair
// should be less that 20 bytes. Maximum data points could be returned for each
// call can be as many as 2MB / 20B = 100000.
pub const MAX_DATA_PONTS_CANISTER_RESPONSE: usize = 100000;

// Remote fetch interval in secs. It is only the canister returned interval
// that is dynamic according to the data size needs to be returned.
pub const REMOTE_FETCH_GRANULARITY: u64 = 60;

// For how many rounds of heartbeat, make a http_request call.
pub const RATE_LIMIT_FACTOR: usize = 5;

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

// Canister heartbeat. Process one item in queue
#[heartbeat]
async fn heartbeat() {
    let mut should_fetch = false;
    RATE_COUNTER.with(|counter| {
        let state = counter.clone().into_inner();
        if state == 0 {
            should_fetch = true;
        }
        counter.replace((state + 1) % RATE_LIMIT_FACTOR);
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

/*
Triggered by heartbeat() function to pick up the next job in the pipe for remote service call.
 */
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

/*
A function to call IC http_request function with sample interval of REMOTE_FETCH_GRANULARITY seconds. Each API
call fetches DATA_POINTS_PER_API data points, which is equivalent of DATA_POINTS_PER_API minutes of data.
 */
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

    let request = CanisterHttpRequestArgs {
        url: url,
        http_method: HttpMethod::GET,
        body: None,
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform_method_name: Some("transform".to_string()),
        headers: request_headers,
    };

    let body = candid::utils::encode_one(&request).unwrap();
    ic_cdk::api::print(format!("Making IC http_request call {} now.", job));

    match ic_cdk::api::call::call_raw(
        Principal::management_canister(),
        "http_request",
        &body[..],
        2_000_000_000,
    )
    .await
    {
        Ok(result) => {
            // decode the result
            let decoded_result: CanisterHttpResponsePayload =
                candid::utils::decode_one(&result).expect("IC http_request failed!");
            // put the result to hashmap
            FETCHED.with(|fetched| {
                let mut fetched = fetched.borrow_mut();
                let decoded_body = String::from_utf8(decoded_result.body)
                    .expect("Remote service response is not UTF-8 encoded.");
                decode_body_to_rates(&decoded_body, &mut fetched);
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

fn decode_body_to_rates(body: &str, fetched: &mut RefMut<HashMap<u64, f32>>) {
    //ic_cdk::api::print(format!("Got decoded result: {}", body));
    let rates_array: Vec<Vec<Value>> = serde_json::from_str(&body).unwrap();
    for rate in rates_array {
        let timestamp = rate[0].as_u64().expect("Couldn't parse the timestamp.");
        let close_rate = rate[4].as_f64().expect("Couldn't parse the rate.");
        fetched.insert(timestamp as Timestamp, close_rate as Rate);
    }
}

#[query]
async fn transform(raw: CanisterHttpResponsePayload) -> CanisterHttpResponsePayload {
    let mut sanitized = raw.clone();
    sanitized.headers = vec![];
    sanitized
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
        let results = RefCell::new(HashMap::<Timestamp, Rate>::new());
        let mut fetched = results.borrow_mut();
        decode_body_to_rates(body, &mut fetched);
        assert!(fetched.len() == 3);
        assert!(fetched.get(&1652454180) == Some(&(9.56 as f32)));
    }
}
