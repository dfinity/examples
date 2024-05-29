use crate::future::PollTwiceFuture;
use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

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

#[update]
async fn update_with_made_up_future_and_panicking_callback() {
    let _guard = scopeguard::guard((), |_| {
        STATE.with(|state| state.borrow_mut().value = Some("guard executed".to_string()));
    });
    let shared_state = Arc::new(Mutex::new(future::SharedState::default()));
    let fut = PollTwiceFuture {
        shared_state: shared_state.clone(),
    };

    let res = fut.await;
    assert_eq!(res, "PollTwiceFuture completed".to_string());

    let _ = shared_state.lock().unwrap().waker.take().unwrap().wake();

    STATE.with(|state| {
        state.borrow_mut().value = Some(format!(
            "in the call back. Will be reverted, due to panic below. Result of future: {:?}",
            res
        ))
    });
    panic!("panicking callback!")
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

mod future {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};

    #[derive(Default)]
    pub(crate) struct SharedState {
        polled: bool,
        pub(crate) waker: Option<Waker>,
    }

    #[derive(Default)]
    pub(crate) struct PollTwiceFuture {
        pub shared_state: Arc<Mutex<SharedState>>,
    }

    impl Future for PollTwiceFuture {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut shared_state = self.shared_state.lock().unwrap();
            ic_cdk::println!("PollTwiceFuture polled once: {}", shared_state.polled);
            if shared_state.polled {
                Poll::Ready("PollTwiceFuture completed".to_string())
            } else {
                shared_state.polled = true;
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
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
