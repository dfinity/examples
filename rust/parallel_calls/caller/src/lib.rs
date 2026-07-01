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
                // icp-cli injects PUBLIC_CANISTER_ID:callee after deploying the callee canister.
                // In multi-subnet PocketIC tests the same env var is injected via
                // CanisterSettings::environment_variables before any calls are made.
                let id = ic_cdk::api::env_var_value("PUBLIC_CANISTER_ID:callee");
                Principal::from_text(&id).expect("invalid PUBLIC_CANISTER_ID:callee")
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
