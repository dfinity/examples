use candid::{CandidType, Decode, Encode, Principal};
use pocket_ic::common::rest::RawMessageId;
use pocket_ic::{PocketIc, RejectResponse};

/// Path to the compiled canister WASM (relative to this file: backend/tests/ -> workspace root -> target/).
pub const CANISTER_WASM: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/backend.wasm");

#[derive(CandidType, Debug, PartialEq, Eq)]
pub enum FutureType {
    TrueAsyncCall,
    FalseAsyncCall,
}

pub struct CanisterSetup {
    pub env: PocketIc,
    canister_id: Principal,
}

impl Default for CanisterSetup {
    fn default() -> Self {
        Self::new()
    }
}

impl CanisterSetup {
    pub fn new() -> Self {
        let env = PocketIc::new();
        let canister_id = env.create_canister();
        env.add_cycles(canister_id, u128::MAX);
        env.install_canister(canister_id, CANISTER_WASM.to_vec(), vec![], None);
        Self { env, canister_id }
    }

    pub fn is_item_processed(&self, item: &str) -> Option<bool> {
        let bytes = self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "is_item_processed",
                Encode!(&item).unwrap(),
            )
            .expect("failed to query is_item_processed");
        Decode!(&bytes, Option<bool>).unwrap()
    }

    pub fn set_non_processed_items(&self, values: &[&str]) {
        let values: Vec<String> = values.iter().map(|s| s.to_string()).collect();
        self.env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "set_non_processed_items",
                Encode!(&values).unwrap(),
            )
            .expect("failed to set non-processed items");
    }

    pub fn process_single_item_with_panicking_callback(
        &self,
        item: &str,
        future_type: &FutureType,
    ) {
        let err = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "process_single_item_with_panicking_callback",
                Encode!(&item, future_type).unwrap(),
            )
            .expect_err("expected canister to trap with 'panicking callback!'");
        assert!(
            err.reject_message.contains("panicking callback!"),
            "unexpected trap message: {}",
            err.reject_message
        );
    }

    pub fn submit_process_single_item_with_panicking_callback(
        &self,
        item: &str,
        future_type: &FutureType,
    ) -> Result<RawMessageId, RejectResponse> {
        self.env.submit_call(
            self.canister_id,
            Principal::anonymous(),
            "process_single_item_with_panicking_callback",
            Encode!(&item, future_type).unwrap(),
        )
    }

    pub fn process_all_items_with_panicking_callback(
        &self,
        panicking_item: &str,
        future_type: &FutureType,
    ) {
        let err = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "process_all_items_with_panicking_callback",
                Encode!(&panicking_item, future_type).unwrap(),
            )
            .expect_err("expected canister to trap with 'panicking callback!'");
        assert!(
            err.reject_message.contains("panicking callback!"),
            "unexpected trap message: {}",
            err.reject_message
        );
    }
}

/// The guard fires via `call_on_cleanup` after a true async call crosses a message boundary:
/// the item is marked as processed even though the callback panicked.
#[test]
fn should_process_single_item_and_mark_it_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);
    assert_eq!(canister.is_item_processed("mint"), Some(false));

    canister.process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall);

    assert_eq!(canister.is_item_processed("mint"), Some(true));
}

/// Without a true async boundary the entire function runs in one message, so a panic
/// rolls back all state changes — the guard has no effect.
#[test]
fn should_process_single_item_but_fail_to_mark_it_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);
    assert_eq!(canister.is_item_processed("mint"), Some(false));

    canister.process_single_item_with_panicking_callback("mint", &FutureType::FalseAsyncCall);

    assert_eq!(canister.is_item_processed("mint"), Some(false));
}

/// When processing multiple items, the first item's guard fires correctly; the panicking
/// item itself is not marked as processed.
#[test]
fn should_process_all_items_and_mark_the_first_one_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint1", "mint2", "mint3"]);

    canister.process_all_items_with_panicking_callback("mint2", &FutureType::TrueAsyncCall);

    assert_eq!(canister.is_item_processed("mint1"), Some(true));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));
}

/// Without a true async boundary, no item is marked as processed even across multiple items.
#[test]
fn should_process_all_items_but_fail_to_mark_the_first_one_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint1", "mint2", "mint3"]);

    canister.process_all_items_with_panicking_callback("mint2", &FutureType::FalseAsyncCall);

    assert_eq!(canister.is_item_processed("mint1"), Some(false));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));
}

/// Submitting two calls for the same item concurrently: one must be rejected with
/// "Item already in processing!" because the in-processing guard prevents it.
/// This test can only be expressed using PocketIC's `submit_call`/`await_call` API,
/// which allows sending calls without waiting for prior ones to complete.
#[test]
fn should_prevent_parallel_processing() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);

    let msg_1 = canister
        .submit_process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall)
        .expect("failed to submit first call");

    let msg_2 = canister
        .submit_process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall)
        .expect("failed to submit second call");

    let result_1 = canister.env.await_call(msg_1);
    let result_2 = canister.env.await_call(msg_2);

    // Exactly one call must fail with "Item already in processing!";
    // the other fails with "panicking callback!" after the guard fires.
    let has_parallel_rejection = [&result_1, &result_2].iter().any(|r| {
        matches!(r, Err(e) if e.reject_message.contains("ERROR: Item already in processing!"))
    });
    assert!(
        has_parallel_rejection,
        "expected one call to be rejected with 'Item already in processing!', got: {:?} / {:?}",
        result_1,
        result_2
    );
}
