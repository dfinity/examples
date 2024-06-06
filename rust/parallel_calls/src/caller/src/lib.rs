use candid::Principal;
use futures::future::{self, BoxFuture};
use ic_cdk::api::call::call;
use ic_cdk::export_candid;
use std::cell::RefCell;

thread_local! {
    static CALLEE: RefCell<Option<Principal>> = RefCell::new(None);
}

#[ic_cdk::update]
pub async fn setup_callee(id: Principal) {
    CALLEE.with(|callee| {
        *callee.borrow_mut() = Some(id);
    });
}

#[ic_cdk::update]
pub async fn sequential_calls(n: u64) -> u64 {
    let mut successful_calls = 0;
    let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());
    for _i in 0..n {
        let r: Result<(), _> = call(callee.clone(), "ping", ()).await;
        match r {
            Ok(_j) => successful_calls += 1,
            Err(_) => {}
        }
    }
    successful_calls
}

#[ic_cdk::update]
pub async fn parallel_calls(n: u64) -> u64 {
    let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

    let mut calls: Vec<BoxFuture<Result<(), _>>> = vec![];
    for _i in 0..n {
        // Note that calls are very likely to start failing for a large enough n (e.g. 1000)
        // Under high load, similar failures can occur with a much lower number of parallel calls,
        // or even with sequential calls.
        // If you need to retry calls, currently the best option is to record the information that a retry is needed,
        // and then perform retries in a timer or a heartbeat.
        calls.push(Box::pin(call(callee, "ping", ())));
    }
    let results = future::join_all(calls).await; // wait for all calls
    results.iter().filter(|r| r.is_ok()).count() as u64
}

export_candid!();
