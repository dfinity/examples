use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
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

thread_local! {
    static STORE: RefCell<SubscriberStore> = RefCell::default();
}

#[update]
fn subscribe(subscriber: Subscriber) -> bool {
    let subscriber_principal_id = ic_cdk::caller();
    STORE.with(|subscriber_store| {
        subscriber_store
            .borrow_mut()
            .entry(subscriber_principal_id)
            .or_insert(subscriber);
    });
    true
}

#[update]
async fn publish(counter: Counter) {
    STORE.with(|subscriber_store| {
        for (k, v) in &*subscriber_store.borrow() {
            if v.topic == counter.topic {
                let _ = ic_cdk::api::call::call::<_, ()>(*k, "update_count", (&counter,));
            }
        }
    })
}
