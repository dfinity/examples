use candid::{candid_method, CandidType, Principal};
use ic_cdk::{self};
use ic_cdk_macros::{self, heartbeat, query, update};
use queues::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type Timestamp = u64;
type Rate = String;
// type Rate = f32;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Clone, Debug, PartialEq, CandidType, Eq, Serialize, Deserialize)]
pub struct RatesWithInterval {
    pub interval: usize,
    pub rates: HashMap<Timestamp, Rate>
}

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

// How many data point can be returned as maximum. 
// Given that 2MB is max-allow cansiter response size, and each <Timestamp, Rate> pair
// should be less that 100 bytes. Maximum data points could be returned for each
// call can be as many as 2MB / 100B = 20000. 
pub const MAX_DATA_PONTS_COUNT: usize = 20000;

// Remote fetch interval is always 60 secs. It is only the canister returned interval
// that is dynamic according to the data size needs to be returned.
pub const REMOTE_FETCH_INTERVAL: i64 = 60;  

thread_local! {
    pub static FETCHED: RefCell<HashMap<Timestamp, String>>  = RefCell::new(HashMap::new());
    pub static REQUESTED: RefCell<Queue<Timestamp>> = RefCell::new(Queue::new());
    pub static HEARTBEAT_COUNT: RefCell<i32> = RefCell::new(0);

    pub static RESPONSE_HEADERS_SANTIZATION: Vec<&'static str> = vec![
        "Date",                     // DateTime of the request is made
        "CF-Cache-Status",          // CloudFront caching status
        "CF-RAY",                   // CloudFront custom Id
        "Age",                      // Age of the data object since query
        "Content-Security-Policy",  // Long list of allowable domains for reference
        "Last-Modified",            // Last time the object is modified
        "Set-Cookie"                // cf-country=US;Path=/;
    ];
}

// Canister heartbeat. Process one item in queue
// #[export_name = "canister_heartbeat"]
#[heartbeat]
async fn heartbeat() {
    get_next_rate().await;
}

/*
Get rates for a time range defined by start time and end time. This function can be invoked
as HTTP update call.
*/
#[update]
#[candid_method(update)]
async fn get_rates(range: TimeRange) -> RatesWithInterval {
    // round down start time and end time to the minute (chop off seconds), to be checked in the hashmap

    // normalize the start_time and end_time to the minute before query remote.
    let start_min = range.start / 60;
    let end_min = range.end / 60;

    // compose a return structure
    let mut fetched = HashMap::new();

    // pull available ranges from hashmap
    FETCHED.with(|map_lock| {
        let map = map_lock.borrow();
        ic_cdk::api::print("In Fetched.");
        for requested_min in start_min..end_min {
                ic_cdk::api::print("In iterations");

            let requested = requested_min * 60;
            if map.contains_key(&requested) {
                // The fetched slot is within user requested range. Add to result for later returning.
                ic_cdk::api::print(format!("Found {} in map!", requested));
                fetched.insert(requested, map.get(&requested).unwrap().clone().to_string());
            } else {
                ic_cdk::api::print(format!("Did not find {} in map!", requested));
                // asynchoronously request downloads for unavailable ranges

                // Simply putting the request to request queue. This queue will likely
                // have duplicate entries, if users request same range of data multiple times.
                // Double downloading is avoided right before the time of downloading by checking
                // whether the data already exists in FETCHED map.
                add_job_to_queue(requested);
            }
        }
    });

    // return sampled rates for available ranges
    sample_with_interval(fetched)
}

fn sample_with_interval(fetched: HashMap<Timestamp, String>) -> RatesWithInterval {
    // in order to make sure that returned data do not exceed 2MB, which is about
    // ~1M data points, calculate interval when data points count is beyond 900K.
    let interval_options = vec![
        1,         // 1 data point every minute
        5,         // 1 data point every 5 minutes
        15,        // 1 data point every 15 minutes
        60,        // 1 data point every hour
        60 * 12,   // 1 data point every 12 hours
        60 * 24    // 1 data point every day
    ];
    for i in interval_options {
        if fetched.len() / i < MAX_DATA_PONTS_COUNT {
            return RatesWithInterval {
                interval: i * 60,
                rates: fetched.into_iter().filter(|r| r.0 % (i as u64) == 0).collect()
            };
        }
    }
    panic!("This shouldn't be happening! Couldn't find a inteval that can keep total data points count in {}.", MAX_DATA_PONTS_COUNT);
}

fn add_job_to_queue(job: Timestamp) -> () {
    REQUESTED.with(|requested_lock| {
        let mut queue = requested_lock.borrow_mut();
        match queue.add(job) {
            Ok(_) => {
                ic_cdk::api::print(format!("Added {} to queue.", job));
            }
            Err(failure) => {
                ic_cdk::api::print(format!("Wasn't able to add job {} to queue. Receiving error {}", job, failure));
            }
        }
    });
}

/*
Register the cron job which take the tip of the queue, and send a canister call to self.
Potentially, different nodes executing the canister will trigger different job during the same period.
The idea is to gap the cron job with large enough time gap, so they won't trigger remove service side
rate limiting.
 */
// #[update]
// #[candid_method(update)]
async fn get_next_rate() -> Option<String> {
    let mut job_id: u64 = 0;

    // Get the next downloading job
    REQUESTED.with(|requested_lock| {
        let mut requested = requested_lock.borrow_mut();

        if requested.size() == 0 {
            ic_cdk::api::print("Request queue empty, no more jobs to fetch.");
            return;
        }

        let job = requested.remove();
        match job {
            Ok(valid_job) => {
                // Job is a valid Job Id
                job_id = valid_job;
                
                FETCHED.with(|fetched_lock| {
                    match fetched_lock.borrow().get(&valid_job) {
                        Some(_) => {
                            // If this job has already been downloaded. Only downloading it if doesn't already exist.
                            ic_cdk::api::print(format!("Rate for {} is already downloaded. Skipping downloading again.", valid_job));
                            return;
                        }
                        None => {
                            // The requested time rate isn't found in map. Send a canister get_rate call to self
                            ic_cdk::api::print(format!("Fetching job {} now.", valid_job));
                        }
                    }
                });
            }
            Err(weird_job) => {
                ic_cdk::api::print(format!("Invalid job found in the request queue! The job Id should be a Unix timestamp divided by 60, e.g., represents a timestamp rounded to minute. Wrong Job Id: {}", weird_job));
                return;
            }
        }
    });
    if job_id != 0 {
        let rate = get_rate(job_id).await;
        return rate;
    }
    return None;
}

/*
A function to call IC http_request function with a single minute range.
This function is to be triggered by timer as jobs move to the tip of the queue.
 */
async fn get_rate(job: Timestamp) -> Option<String> {
    let start_timestamp = job as i64;
    let end_timestamp = (job + 59) as i64;

    let host = "pro.coinbase.com";
    // prepare system http_request call
    let mut request_headers = vec![];
    request_headers.insert(
        0,
        HttpHeader {
            name: "Host".to_string(),
            value: host.to_string(),
        },
    );
    let url = format!("https://api.{host}/products/ICP-USD/candles?granularity=60&start={start_timestamp}&end={end_timestamp}");
    ic_cdk::api::print(url.clone());

    let request = CanisterHttpRequestArgs {
        url: url,
        http_method: HttpMethod::GET,
        body: None,
        transform_method_name: Some("transform".to_string()),
        headers: request_headers,
    };

    let body = candid::utils::encode_one(&request).unwrap();
    ic_cdk::api::print(format!("Making IC http_request call {} now.", job));

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
            let decoded_result: String = candid::utils::decode_one(&result).unwrap();
            ic_cdk::api::print(format!("Got decoded result: {}", decoded_result));
            // put the result to hashmap
            FETCHED.with(|fetched_lock| {
                let mut stored = fetched_lock.borrow_mut();
                stored.insert(job, decoded_result.clone());
            });
            return Some(decoded_result);
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            ic_cdk::api::print(message.clone());
            // TODO - Remove this. Putting the result to hashmap for debuging purpose
            FETCHED.with(|fetched_lock| {
                let mut stored = fetched_lock.borrow_mut();
                stored.insert(job, message.clone());
            });
            
            // Since the remote request failed. Adding the de-queued job back again for retries.
            add_job_to_queue(job);
            return Some(message);
        }
    }
}

#[query]
#[candid_method(query)]
#[export_name = "transform"]
async fn transform(raw: CanisterHttpResponsePayload) -> CanisterHttpResponsePayload {
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

fn main() {}
