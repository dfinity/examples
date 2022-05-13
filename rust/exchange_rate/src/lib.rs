use candid::{candid_method, CandidType, Principal};
use ic_cdk;
use ic_cdk_macros::{self, query, update};
use queues::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type Timestamp = i64;
type Rate = f32;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, CandidType, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterHttpRequestArgs {
    pub url: String,
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

thread_local! {
    pub static FETCHED: RefCell<HashMap<Timestamp, Rate>>  = RefCell::new(HashMap::new());
    pub static REQUESTED: RefCell<Queue<Timestamp>> = RefCell::new(Queue::new());

    pub static RESPONSE_HEADERS_SANTIZATION: Vec<&'static str> = vec![
        "x-mbx-uuid",
        "x-mbx-used-weight",
        "x-mbx-used-weight-1m",
        "Date",
        "Via",
        "X-Amz-Cf-Id",
    ];
}

/// Canister heartbeat. Process one item in queue
#[export_name = "canister_heartbeat"]
async fn heartbeat() {
    get_next_rate().await;
}

/*
Get rates for a time range defined by start time and end time. This function can be invoked
as HTTP update call.
*/
#[update]
#[candid_method(update)]
async fn get_rates(start: Timestamp, end: Timestamp) -> HashMap<Timestamp, Rate> {
    // round down start time and end time to the minute (chop off seconds), to be checked in the hashmap
    let start_min = start / 60;
    let end_min = end / 60;

    // compose a return structure
    let mut fetched = HashMap::new();

    // pull available ranges from hashmap
    FETCHED.with(|map_lock| {
        let map = map_lock.borrow();
        for requested_min in start_min..end_min {
            let requested = requested_min * 60;
            if map.contains_key(&requested) {
                // The fetched slot is within user requested range. Add to result for later returning.
                fetched.insert(requested, map.get(&requested).unwrap().clone());
            } else {
                // asynchoronously request downloads for unavailable ranges

                // Simply putting the request to request queue. This queue will likely
                // have duplicate entries, if users request same range of data multiple times.
                // Double downloading is avoided right before the time of downloading by checking
                // whether the data already exists in FETCHED map.
                REQUESTED.with(|requested_lock| {
                    let mut queue = requested_lock.borrow_mut();
                    match queue.add(requested) {
                        Ok(_) => {}
                        Err(failure) => {
                            println!(
                                "Wasn't able to add job {} to queue. Receiving error {}",
                                requested, failure
                            );
                        }
                    }
                });
            }
        }
    });

    // return rates for available ranges
    return fetched;
}

/*
Register the cron job which take the tip of the queue, and send a canister call to self.
Potentially, different nodes executing the canister will trigger different job during the same period.
The idea is to gap the cron job with large enough time gap, so they won't trigger remove service side
rate limiting.
 */
async fn get_next_rate() -> () {
    let mut job_id: i64 = 0;

    // Get the next downloading job
    REQUESTED.with(|requested_lock| {
        let mut requested = requested_lock.borrow_mut();

        if requested.size() == 0 {
            return;
        }

        let job = requested.remove();

        match job {
            Ok(valid_job) => {
                // Job is a valid Job Id
                FETCHED.with(|fetched_lock| {
                    match fetched_lock.borrow().get(&valid_job) {
                        Some(_) => {
                            // If this job has already been downloaded. Only downloading it if doesn't already exist.
                            println!("Rate for {} is already downloaded. Skipping downloading again.", valid_job);
                            return;
                        }
                        None => {
                            // The requested time rate isn't found in map. Send a canister get_rate call to self
                            println!("Fetching job {valid_job} now.");
                            job_id = valid_job;
                        }
                    }
                });

            }
            Err(weird_job) => {
                println!("Invalid job found in the request queue! The job Id should be a Unix timestamp divided by 60, e.g., represents a timestamp rounded to minute. Wrong Job Id: {weird_job}");
                return;
            }
        }

    });
    get_rate(job_id).await;
}

/*
A function to call IC http_request function with a single minute range.
This function is to be triggered by timer as jobs move to the tip of the queue.
 */
async fn get_rate(key: Timestamp) {
    let start_time = key * 1000;
    let end_time = (key + 1) * 1000 - 1;

    // prepare system http_request call
    let mut request_headers = vec![];
    request_headers.insert(
        0,
        HttpHeader {
            name: "Connection".to_string(),
            value: "keep-alive".to_string(),
        },
    );

    let request = CanisterHttpRequestArgs {
        url: format!("https://api.binance.com/api/v3/klines?symbol=ICPUSDT&interval=1m&startTime={start_time}&endTime={end_time}"),
        http_method: HttpMethod::GET,
        body: None,
        transform_method_name: Some("sanitize_response".to_string()),
        headers: request_headers,
    };

    let body = candid::utils::encode_one(&request).unwrap();

    match ic_cdk::api::call::call_raw(
        Principal::management_canister(),
        "http_request",
        &body[..],
        0,
    )
    .await
    {
        Ok(result) => {
            // decode the result
            let decoded_result = candid::utils::decode_one(&result).unwrap();

            // put the result to hashmap
            FETCHED.with(|fetched_lock| {
                let mut stored = fetched_lock.borrow_mut();
                stored.insert(key, decoded_result);
            });
        }
        Err((r, m)) => {
            println!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
        }
    }
}

#[query]
#[candid_method(query)]
#[export_name = "transform"]
async fn sanitize_response(raw: CanisterHttpResponsePayload) -> CanisterHttpResponsePayload {
    let mut sanitized = raw.clone();
    RESPONSE_HEADERS_SANTIZATION.with(|response_headers_blacklist| {
        let mut processed_headers = vec![];
        for header in raw.headers.iter() {
            if !response_headers_blacklist.contains(&header.name.as_str()) {
                processed_headers.insert(0, header.clone());
            }
        }
        sanitized.headers = processed_headers;
    });
    return sanitized;
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}
