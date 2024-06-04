use crate::parallel_guard::ParallelProcessingGuard;
use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct State {
    /// Bunch of items, where the boolean value indicates whether this item was processed.
    items: BTreeMap<String, bool>,
    /// Items that are currently being processed.
    processing_items: BTreeSet<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[update]
async fn process_single_item_with_panicking_callback(
    item_to_process: String,
    future_type: FutureType,
) {
    let _prevent_parallel_calls_guard = ensure_not_in_processing(item_to_process.clone());
    ensure_not_processed(&item_to_process);
    let _process_item_only_once_guard = scopeguard::guard((), |_| {
        STATE.with(|state| {
            state
                .borrow_mut()
                .items
                .entry(item_to_process)
                .and_modify(|v| *v = true);
        });
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

    panic!("panicking callback!")
}

#[update]
async fn process_all_items_with_panicking_callback(
    panicking_item: String,
    future_type: FutureType,
) {
    let items: Vec<String> = STATE.with(|state| state.borrow().items.keys().cloned().collect());
    for item in items {
        let _prevent_parallel_calls_guard = ensure_not_in_processing(item.clone());
        ensure_not_processed(&item);
        let _process_item_only_once_guard = scopeguard::guard((), |_| {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .items
                    .entry(item.clone())
                    .and_modify(|v| *v = true);
            });
        });
        // Note that this is the callback of the previous item. Similarly to the above method,
        // the same 2 scenarios apply and whether the guard is effective or not depends on whether
        // the future was polled until completion in a single message.
        if item == panicking_item {
            panic!("panicking callback!");
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

#[query]
fn is_item_processed(item: String) -> Option<bool> {
    STATE.with(|state| state.borrow().items.get(&item).cloned())
}

#[update]
fn set_non_processed_items(values: Vec<String>) {
    ensure_none_in_processing();
    STATE.with(|state| {
        state.borrow_mut().items = values.into_iter().map(|item| (item, false)).collect();
    });
}

fn ensure_not_processed(item: &str) {
    if let Some(true) = is_item_processed(item.to_string()) {
        panic!("ERROR: Item '{}' already processed!", item);
    }
}

fn ensure_not_in_processing(item: String) -> ParallelProcessingGuard {
    ParallelProcessingGuard::new(item).expect("ERROR: Item already in processing!")
}

fn ensure_none_in_processing() {
    STATE.with(|state| {
        assert_eq!(
            state.borrow().processing_items,
            BTreeSet::default(),
            "ERROR: Items already in processing!"
        );
    });
}

mod parallel_guard {
    use crate::STATE;

    pub struct ParallelProcessingGuard {
        item: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ParallelProcessingGuardError {
        AlreadyProcessing,
    }

    impl ParallelProcessingGuard {
        pub fn new(item: String) -> Result<Self, ParallelProcessingGuardError> {
            STATE.with(|state| {
                if !state.borrow_mut().processing_items.insert(item.clone()) {
                    return Err(ParallelProcessingGuardError::AlreadyProcessing);
                }
                Ok(Self { item })
            })
        }
    }

    impl Drop for ParallelProcessingGuard {
        fn drop(&mut self) {
            STATE.with(|state| {
                state.borrow_mut().processing_items.remove(&self.item);
            });
        }
    }
}
