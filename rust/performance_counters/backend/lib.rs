//! An example of performance counters usage.

// The following performance counters are supported:
//
// - 0 : current execution instruction counter.
//       The number of WebAssembly instructions the canister has executed
//       since the beginning of the current Message execution.
//
// - 1 : call context instruction counter.
//       - For replicated message execution, it is the number of WebAssembly instructions
//         the canister has executed within the call context of the current Message execution
//         since Call context creation. The counter monotonically increases across all message
//         executions in the call context until the corresponding call context is removed.
//       - For non-replicated message execution, it is the number of WebAssembly instructions
//         the canister has executed within the corresponding `composite_query_helper`
//         in Query call. The counter monotonically increases across the executions
//         of the composite query method and the composite query callbacks
//         until the corresponding `composite_query_helper` returns
//         (ignoring WebAssembly instructions executed within any further downstream calls
//         of `composite_query_helper`).
//
// In the future, ICP might expose more performance counters.
use ic_cdk::api::{call_context_instruction_counter, canister_self, instruction_counter, performance_counter};
use ic_cdk::call::Call;

/// Pretty print the `title` and a corresponding `tuple` with counters.
fn pretty_print<N: std::fmt::Display, T: std::fmt::Display>(title: N, counters: (T, T)) {
    ic_cdk::println!("{:40} {:<15} {:<15}", title, counters.0, counters.1);
}

/// Loop to simulate some amount of work.
fn do_some_work() {
    for i in 0..1_000_000 {
        // The black box hint is to avoid compiler optimizations for the loop.
        std::hint::black_box(i);
    }
}

/// Returns a tuple with all the performance counters.
fn counters() -> (u64, u64) {
    (instruction_counter(), call_context_instruction_counter())
}

/// Emulate a nested inter-canister update call.
#[ic_cdk::update]
fn nested_update_call() -> (u64, u64) {
    counters()
}

/// Emulate a nested inter-canister composite query call.
#[ic_cdk::query(composite = true)]
fn nested_composite_query_call() -> (u64, u64) {
    counters()
}

/// Emulate a nested inter-canister query call.
#[ic_cdk::query]
fn nested_call() {}

////////////////////////////////////////////////////////////////////////
// Canister interface
////////////////////////////////////////////////////////////////////////

/// Example usage: `icp canister call backend for_update`
#[ic_cdk::update]
async fn for_update() -> (u64, u64) {
    do_some_work();
    let before = counters();

    let inside_1st: (u64, u64) = Call::unbounded_wait(canister_self(), "nested_update_call")
        .await
        .expect("nested_update_call failed")
        .candid_tuple::<(u64, u64)>()
        .expect("Candid decoding failed");

    do_some_work();
    let after_1st = counters();

    let inside_2nd: (u64, u64) = Call::unbounded_wait(canister_self(), "nested_update_call")
        .await
        .expect("nested_update_call failed")
        .candid_tuple::<(u64, u64)>()
        .expect("Candid decoding failed");

    do_some_work();
    let after_2nd = counters();

    pretty_print(
        "Performance counters for update call:",
        ("current (0)", "call context (1)"),
    );
    pretty_print("  before the nested call:", before);
    pretty_print("  > inside the 1st nested call:", inside_1st);
    pretty_print("  after the 1st nested call:", after_1st);
    pretty_print("  > inside the 2nd nested call:", inside_2nd);
    pretty_print("  after the 2nd nested call:", after_2nd);

    after_2nd
}

/// Example usage: `icp canister call --query backend for_composite_query`
#[ic_cdk::query(composite = true)]
async fn for_composite_query() -> (u64, u64) {
    do_some_work();
    let before = counters();

    let inside_1st: (u64, u64) =
        Call::unbounded_wait(canister_self(), "nested_composite_query_call")
            .await
            .expect("nested_composite_query_call failed")
            .candid_tuple::<(u64, u64)>()
            .expect("Candid decoding failed");

    do_some_work();
    let after_1st = counters();

    let inside_2nd: (u64, u64) =
        Call::unbounded_wait(canister_self(), "nested_composite_query_call")
            .await
            .expect("nested_composite_query_call failed")
            .candid_tuple::<(u64, u64)>()
            .expect("Candid decoding failed");

    do_some_work();
    let after_2nd = counters();

    pretty_print(
        "Perf. counters for composite query call:",
        ("current (0)", "call context (1)"),
    );
    pretty_print("  before the nested call:", before);
    pretty_print("  > inside the 1st nested call:", inside_1st);
    pretty_print("  after the 1st nested call:", after_1st);
    pretty_print("  > inside the 2nd nested call:", inside_2nd);
    pretty_print("  after the 2nd nested call:", after_2nd);

    after_2nd
}

/// Example usage: `icp canister call --query backend example`
#[ic_cdk::query(composite = true)]
async fn example() -> (u64, u64) {
    do_some_work();
    Call::unbounded_wait(canister_self(), "nested_call")
        .await
        .expect("nested_call failed");

    do_some_work();
    Call::unbounded_wait(canister_self(), "nested_call")
        .await
        .expect("nested_call failed");

    do_some_work();
    (performance_counter(0), performance_counter(1))
}
ic_cdk::export_candid!();
