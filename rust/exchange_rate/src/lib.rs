use candid::{candid_method, CandidType, Error, Principal};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dfn_candid::candid_one;
use dfn_core::{over_async, print};
use ic_cdk;
use ic_cdk::export::candid::{candid_method, CandidType};
use ic_cdk_macros;
use ic_ic00_types::{CanisterHttpRequestArgs, CanisterHttpResponsePayload, HttpHeader, HttpMethod};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::SystemTime;

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
    pub static FETCHED: RefCell<HashMap<SystemTime, Rate>>  = RefCell::new(HashMap::new());
    pub static LAST_FETCHED_TIME: SystemTime = SystemTime::now();
    pub static REQUESTED: Vec<SystemTime> = vec![];
}

#[ic_cdk_macros::update(name = "get_rates")]
#[candid_method(update, rename = "get_rates")]
async fn fetech_rates(start: SystemTime, end: SystemTime) -> Result<Vec<Rate>, Error> {
    // round down start time and end time to the minute (chop off seconds), to be checked in the hashmap
    let start_min = start / 60;
    let end_min = end / 60;
    let query_range = start_min..end_min;

    // compose a return structure
    let mut fetched = HashMap::new();

    // pull available ranges from hashmap
    for (key, value) in FETCHED.borrow().iter() {
        if query_range.contains(key) {
            fetched.insert(key, value);
        } else {
            // asynchoronously request downloads for unavailable ranges

            // putting the request to request queue, to avoid same record being downloaded
            // multiple times by multiple users
            if !REQUESTED.contains(key) {
                REQUESTED.insert(key);
            }

            // prepare system http_request call
            let request_headers = vec![];
            request_headers.insert(HttpHeader {
                name: "Connection",
                value: "keep-alive",
            });

            let request = CanisterHttpRequestArgs {
                url: format!("https://api.binance.com/api/v3/klines?symbol=ICPUSDT&interval=1m&startTime={}&endTime={}", key * 60 * 1000, key * 60 * 1000 - 1),
                http_method: HttpMethod::GET,
                body: None,
                transform_method_name: "", // TODO: switch to "sanitize_response" once it's created
                headers: request_headers,
            };

            dfn_core::api::print("send_request encoding CanisterHttpRequestArgs message.");
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
    }

    // return rates for available ranges
    return fetched;
}

async fn request_download(start: SystemTime) {}

#[ic_cdk_macros::query(name = "sanitize_response")]
#[candid_method(query, rename = "sanitize_response")]
async fn sanitize_response(
    raw: Result<CanisterHttpResponsePayload, String>,
) -> Result<CanisterHttpResponsePayload, String> {
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
