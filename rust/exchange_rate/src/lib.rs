use candid::{candid_method, CandidType, Error, Principal};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dfn_candid::candid_one;
use dfn_core::{over_async, print};
use ic_cdk;
use ic_cdk_macros;
use queues::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::SystemTime;
use tokio_cron_scheduler::{Job, JobScheduler};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct TimeRange {
    pub start: SystemTime,
    pub end: SystemTime,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct Rate {
    pub value: f32,
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

thread_local! {
    pub static FETCHED: RefCell<HashMap<u64, Rate>>  = RefCell::new(HashMap::new());
    pub static REQUESTED: RefCell<Queue<u64>> = RefCell::new(Queue::new());
    pub static SCHEDULER: JobScheduler = JobScheduler::new().unwrap();
}

#[ic_cdk_macros::update(name = "get_rates")]
#[candid_method(update, rename = "get_rates")]
async fn get_rates(start: SystemTime, end: SystemTime) -> Result<Vec<Rate>, Error> {
    // round down start time and end time to the minute (chop off seconds), to be checked in the hashmap
    let start_min = start
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 60;
    let end_min = end
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 60;

    // compose a return structure
    let mut fetched = HashMap::new();

    // pull available ranges from hashmap
    FETCHED.with(|map_lock| {
        let map = map_lock.borrow();
        for requested in start_min..end_min {
            if map.contains_key(&requested) {
                // The fetched slot is within user requested range. Add to result for later returning.
                fetched.insert(requested, map.get(&requested));
            } else {
                // asynchoronously request downloads for unavailable ranges

                // Simply putting the request to request queue. This queue will likely
                // have duplicate entries, if users request same range of data multiple times.
                // Double downloading is avoided right before the time of downloading by checking
                // whether the data already exists in FETCHED map.
                REQUESTED.with(|requested_lock| {
                    let mut queue = requested_lock.borrow_mut();
                    queue.add(requested);
                });
            }
        }
    });

    // Kick off scheduler if it hasn't already been kicked off
    SCHEDULER.with(|scheduler| {
        //let mut scheduler = scheduler_lock.borrow_mut();
        match scheduler.time_till_next_job() {
            Err(_) => {
                // The scheduler has not been started. Initialize it.
                scheduler.add(
                    Job::new("1/5 * * * * *", |_, _| {
                        register_cron_job();
                    })
                    .unwrap(),
                );
                scheduler.start();
            }
            Ok(_) => {
                println!("Scheduler already started. Skipping initializing again.");
            }
        };
    });

    // return rates for available ranges
    return fetched;
}

async fn register_cron_job() -> () {
    println!("Starting scheduler.");

    // Get the next downloading job
    REQUESTED.with(|requested_lock| {
        let mut requested = requested_lock.borrow_mut();
        let job = requested.remove();

        match job {
            Ok(valid_job) => {
                // Job is a valid Job Id
                FETCHED.with(|fetched_lock| match fetched_lock.borrow().get(&valid_job) {
                    Some(_) => {
                        // If this job has already been downloaded. Only downloading it if doesn't already exist.
                        return;
                    }
                    None => {
                        // The requested time rate isn't found in map. Send a canister get_rate call to self
                    }
                });
            }
            Err(weird_job) => {
                println!("Invalid job found in the request queue! Job: {weird_job}");
            }
        }
    });

    return;
}

#[ic_cdk_macros::update(name = "get_rate")]
#[candid_method(update, rename = "get_rate")]
async fn get_rate(key: SystemTime) {
    // prepare system http_request call
    let request_headers = vec![];
    request_headers.insert(
        0,
        HttpHeader {
            name: "Connection".to_string(),
            value: "keep-alive".to_string(),
        },
    );

    let request = CanisterHttpRequestArgs {
        url: format!("https://api.binance.com/api/v3/klines?symbol=ICPUSDT&interval=1m&startTime={}&endTime={}", key * 60 * 1000, key * 60 * 1000 - 1),
        http_method: HttpMethod::GET,
        body: None,
        transform_method_name: "", // TODO: switch to "sanitize_response" once it's created
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

            // put the result to hashmap
            let stored = FETCHED.borrow_mut();
            stored.insert(key, result);
            REQUESTED.remove(key);
        }
        Err() => {
            // log
        }
    }
}

#[ic_cdk_macros::query(name = "sanitize_response")]
#[candid_method(query, rename = "sanitize_response")]
async fn sanitize_response(
    raw: Result<CanisterHttpResponsePayload, Error>,
) -> Result<CanisterHttpResponsePayload, Error> {
    match raw {
        Ok(mut r) => {
            //
            let mut processed_headers = vec![];
            for header in r.headers.iter() {
                if header.name != "date" {
                    processed_headers.insert(0, header.clone());
                }
            }
            r.headers = processed_headers;
            return Result::Ok(r);
        }
        Err(m) => {
            return Result::Err(m);
        }
    }
}

fn main() {}
