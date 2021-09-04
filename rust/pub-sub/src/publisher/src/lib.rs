use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::BTreeMap;

type SubscriberStore = BTreeMap<Principal, Subscriber>;

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
fn subscribe(subscriber: Subscriber) -> bool {
    let subscriber_principal_id = ic_cdk::caller();
    let subscriber_store = storage::get_mut::<SubscriberStore>();
    if !subscriber_store.contains_key(&subscriber_principal_id) {
        subscriber_store.insert(subscriber_principal_id, subscriber);
    }
    true
}

#[update]
async fn publish(counter: Counter) {
    let subscriber_store = storage::get_mut::<SubscriberStore>();
    for (k, v) in subscriber_store {
        if v.topic == counter.topic {
            let _call_result: Result<(), _> =
                ic_cdk::api::call::call(*k, "update_count", (&counter,)).await;
        }
    }
}
