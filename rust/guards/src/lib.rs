use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct State {
    value: Option<String>,
    other_values: BTreeSet<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[update]
async fn update_with_panicking_callback(future_type: FutureType) {
    let _guard = scopeguard::guard((), |_| {
        STATE.with(|state| state.borrow_mut().value = Some("guard executed".to_string()));
    });

    // Call and await future. There are 2 scenarios to consider.
    // 1) The future cannot be polled until completion.
    // The future returns a result of type `futures::task::Poll::Pending` indicating that it's not ready yet.
    // In that case, the state will be committed and execution will yield, terminating the execution of that first message.
    // Execution will then continue in a second message, when the future is ready.
    // That means that the panic at the end will only revert the state changes occurring in the second message.
    // The Rust ic_cdk will during `call_on_cleanup` call `Drop` on any variables that still in scope at the end of the first message,
    // hence the guard will be executed and not reverted.
    //
    // 2) The future can be polled until completion.
    // The future returns a result of type `futures::task::Poll::Ready` and in that case execution will continue without yielding.
    // Everything will be executed in a single message and any state modification will be dropped due to the panic occurring at the end.
    // In that case, the guard is ineffective
    future_type.call().await;

    STATE.with(|state| {
        state.borrow_mut().value =
            Some("in the call back. Will be reverted, due to panic below".to_string())
    });
    panic!("panicking callback!")
}

#[update]
async fn update_multi_async_with_panicking_callback(
    future_type: FutureType,
    panicking_element: String,
) {
    let values: Vec<String> =
        STATE.with(|state| state.borrow().other_values.iter().cloned().collect());
    for value in values.iter() {
        let _elements_should_be_processed_only_once_guard = scopeguard::guard(value, |_| {
            let _is_removed = STATE.with(|state| state.borrow_mut().other_values.remove(value));
        });
        if value == &panicking_element {
            panic!("panicking callback");
        }
        future_type.call().await;
    }
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub enum FutureType {
    TrueAsyncCall,
    FalseAsyncCall,
}

impl FutureType {
    pub async fn call(&self) {
        match self {
            FutureType::TrueAsyncCall => {
                let _ = ic_cdk::api::management_canister::main::raw_rand().await;
            }
            FutureType::FalseAsyncCall => {
                //NOP
            }
        }
    }
}

#[update]
fn reset() {
    STATE.with(|state| *state.borrow_mut() = State::default());
}

#[update]
fn set_values(values: Vec<String>) {
    STATE.with(|state| {
        state.borrow_mut().other_values = values.into_iter().collect();
    });
}

#[query]
fn get_value() -> Option<String> {
    STATE.with(|state| state.borrow().value.clone())
}

#[query]
fn get_values() -> Vec<String> {
    STATE.with(|state| state.borrow().other_values.clone().into_iter().collect())
}
