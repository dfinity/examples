use candid::{CandidType, Principal};
use ic_cdk;
use ic_cdk_macros::*;
use serde::Deserialize;

static mut COUNTER: u64 = 0;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let _call_result: Result<(), _> =
        ic_cdk::api::call::call(publisher_id, "subscribe", (subscriber,)).await;
}

#[update]
fn update_count(counter: Counter) {
    unsafe {
        COUNTER += counter.value;
    }
}

#[query]
fn get_count() -> u64 {
    unsafe { COUNTER }
}
