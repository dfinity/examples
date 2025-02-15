use ic_cdk::call::CallError;
use ic_cdk::prelude::*;
use ic0::msg_deadline as unsafe_msg_deadline;

fn msg_deadline() -> u64 {
    unsafe {
        unsafe_msg_deadline()
    }
}

/// Demonstrates that the system accepts best-effort calls, and that the receiver observes a deadline.
///
/// Invokes another method either with a best-effort response or a guaranteed response call,
/// depending on the value of `use_best_effort_response`. Returns the deadline as observed by the
/// receiver method.
#[ic_cdk::update]
async fn demonstrate_deadlines(use_best_effort_response: bool) -> u64 {
    let call_builder = Call::new(ic_cdk::api::canister_self(), "deadline_in_update").with_args(());
    let call_builder = if use_best_effort_response {
        call_builder.change_timeout(1)
    } else {
        call_builder.with_guaranteed_response()
    };
    let res: u64= call_builder
        .call()
        .await
        .expect("Didn't expect the call to fail");
    res
}

/// Endpoint that demonstrates that timeouts can trigger with best-effort responses.
/// It calls busy() with a timeout of 1 second, which is not enough to complete the
/// execution, triggering a SYS_UNKNOWN error on the call. We return a bool to
/// the caller to indicate whether a timeout has occured (true = yes, false = no)
#[ic_cdk::update]
async fn demonstrate_timeouts() -> bool {
    let res: CallResult<u64> = Call::new(ic_cdk::api::canister_self(), "busy")
        .change_timeout(1)
        .with_args(((),))
        .call()
        .await;
    match res {
        Err(CallError::CallRejected(RejectCode::SysUnknown, s)) => {
            ic_cdk::println!("SysUnknown: {:?}", s);
            true
        }
        Err(e) => {
            ic_cdk::println!("Unexpected error returned by the call: {:?}", e);
            false
        }
        Ok(r) => {
            ic_cdk::println!("Unexpected successful result returned by the call: {:?}", r);
            false
        }
    }
}

/// Busy endpoint that just wastes a lot of instruction to trigger multi-round
/// execution, which in turn can trigger timeouts on best-effort response calls
/// to this endpoint
#[ic_cdk::update]
async fn busy() -> u64 {
    const ROUNDS: u32 = 5;
    const INSTRUCTIONS_PER_SLICE: u32 = 2_000_000_000;
    const TOTAL_INSTRUCTIONS: u64 = ROUNDS as u64 * INSTRUCTIONS_PER_SLICE as u64;

    while ic_cdk::api::performance_counter(0) < TOTAL_INSTRUCTIONS {}
    ic_cdk::api::performance_counter(0)
}

#[ic_cdk::update]
async fn deadline_in_update() -> u64 {
    msg_deadline()
}

#[ic_cdk::query]
async fn deadline_in_query() -> u64 {
    msg_deadline()
}

#[ic_cdk::query(composite = true)]
async fn deadline_in_composite_query() -> u64 {
    msg_deadline()
}

#[ic_cdk::query(composite = true)]
async fn test_deadlines_in_composite_query() -> (u64, u64) {
    let deadline_in_query: u64 = Call::new(ic_cdk::api::canister_self(), "deadline_in_query")
        .with_args(())
        .change_timeout(1)
        .call()
        .await
        .expect("Failed to call deadline_in_query");
    let deadline_of_query_in_composite_query: u64 =
        Call::new(ic_cdk::api::canister_self(), "deadline_in_composite_query")
            .with_args(())
            .change_timeout(1)
            .call()
            .await
            .expect("Failed to call deadline_in_composite_query");
    (deadline_in_query, deadline_of_query_in_composite_query)
}

#[ic_cdk::update]
async fn deadline_in_replicated_query() -> u64 {
    Call::new(ic_cdk::api::canister_self(), "deadline_in_query")
        .with_args(())
        .change_timeout(1)
        .call::<u64>()
        .await
        .expect("Failed to call deadline_in_query")
}
