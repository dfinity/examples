use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::cell::RefCell;

#[derive(Clone, CandidType, Serialize, Deserialize, PartialEq, Eq, Copy, Debug)]
pub enum FnType {
    Heartbeat,
    OnLowWasmMemory,
}

struct State {
    fn_order: Vec<FnType>,
    bytes: Vec<u8>,
    hook_executed: bool,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow().as_ref().expect("state not initialized")))
}

/// A helper method to mutate the state.
///
/// Precondition: the state is already initialized.
fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

// A helper method to set the state.
fn set_state(state: State) {
    STATE.with(|cell| *cell.borrow_mut() = Some(state));
}

/// Initializes the state of the Bitcoin canister.
#[ic_cdk_macros::init]
pub fn init() {
    set_state(State {
        fn_order: vec![],
        bytes: vec![],
        hook_executed: false,
    });
}

#[ic_cdk_macros::query]
pub fn get_executed_functions_order() -> Vec<FnType> {
    with_state(|s| s.fn_order.clone())
}

#[ic_cdk_macros::heartbeat]
pub fn increase_wasm_memory_usage() {
    with_state_mut(|s| {
        if !s.hook_executed {
            s.fn_order.push(FnType::Heartbeat);
            let temp = vec![0u8; 1024];
            for i in temp {
                s.bytes.push(i);
            }
        }
    });
}

#[ic_cdk_macros::on_low_wasm_memory]
pub fn hook() {
    with_state_mut(|s| {
        s.hook_executed = true;
        s.fn_order.push(FnType::OnLowWasmMemory)
    });
}

ic_cdk::export_candid!();
