use candid::Principal;
use ic_cdk::call::Call;
use ic_cdk::export_candid;
use std::cell::RefCell;
use std::future::IntoFuture;

thread_local! {
    static CALLEE: RefCell<Option<Principal>> = RefCell::new(None);
}

fn callee() -> Principal {
    CALLEE.with(|c| {
        c.borrow_mut()
            .get_or_insert_with(|| {
                let id = std::env::var("PUBLIC_CANISTER_ID:callee")
                    .expect("PUBLIC_CANISTER_ID:callee not set");
                Principal::from_text(&id).expect("invalid principal")
            })
            .clone()
    })
}

#[ic_cdk::update]
pub async fn sequential_calls(n: u64) -> u64 {
    let callee = callee();
    let mut successful_calls = 0;
    for _i in 0..n {
        let r = Call::bounded_wait(callee, "ping").await;
        if r.is_ok() {
            successful_calls += 1;
        }
    }
    successful_calls
}

#[ic_cdk::update]
pub async fn parallel_calls(n: u64) -> u64 {
    let callee = callee();

    // Note that calls are very likely to start failing for a large enough n (e.g. 1000).
    // Under high load, similar failures can occur with a much lower number of parallel calls,
    // or even with sequential calls.
    // If you need to retry calls, currently the best option is to record the information that
    // a retry is needed, and then perform retries in a timer or a heartbeat.
    let calls: Vec<_> = (0..n)
        .map(|_| Call::bounded_wait(callee, "ping").into_future())
        .collect();
    let results = futures::future::join_all(calls).await;
    results.iter().filter(|r| r.is_ok()).count() as u64
}

export_candid!();
