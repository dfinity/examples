use assert_matches::assert_matches;
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use pocket_ic::common::rest::RawMessageId;
use pocket_ic::{ErrorCode, PocketIc, UserError, WasmResult};

pub const CANISTER_WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/ic_fun_with_guards.wasm");

#[test]
fn should_process_single_item_and_mark_it_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);
    assert_eq!(canister.is_item_processed("mint"), Some(false));

    canister.process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall);

    assert_eq!(canister.is_item_processed("mint"), Some(true));
}

#[test]
fn should_process_single_item_but_fail_to_mark_it_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);
    assert_eq!(canister.is_item_processed("mint"), Some(false));

    canister.process_single_item_with_panicking_callback("mint", &FutureType::FalseAsyncCall);

    assert_eq!(canister.is_item_processed("mint"), Some(false));
}

#[test]
fn should_process_all_items_and_mark_the_first_one_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint1", "mint2", "mint3"]);
    assert_eq!(canister.is_item_processed("mint1"), Some(false));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));

    canister.process_all_items_with_panicking_callback("mint2", &FutureType::TrueAsyncCall);

    assert_eq!(canister.is_item_processed("mint1"), Some(true));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));
}

#[test]
fn should_process_all_items_but_fail_to_mark_the_first_one_as_processed() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint1", "mint2", "mint3"]);
    assert_eq!(canister.is_item_processed("mint1"), Some(false));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));

    canister.process_all_items_with_panicking_callback("mint2", &FutureType::FalseAsyncCall);

    assert_eq!(canister.is_item_processed("mint1"), Some(false));
    assert_eq!(canister.is_item_processed("mint2"), Some(false));
    assert_eq!(canister.is_item_processed("mint3"), Some(false));
}

#[test]
fn should_prevent_parallel_processing() {
    let canister = CanisterSetup::default();
    canister.set_non_processed_items(&["mint"]);

    let process_item_1 = canister
        .submit_process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall)
        .unwrap();

    let process_item_2 = canister
        .submit_process_single_item_with_panicking_callback("mint", &FutureType::TrueAsyncCall)
        .unwrap();

    let result_1 = canister.env.await_call(process_item_1);
    let result_2 = canister.env.await_call(process_item_2);

    assert_matches!((result_1, result_2),
        (Err(UserError { code, description }), _) | (_, Err(UserError { code, description }))
            if code == ErrorCode::CanisterCalledTrap && description.contains("ERROR: Item already in processing!"));
}

pub struct CanisterSetup {
    env: PocketIc,
    canister_id: CanisterId,
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
        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "is_item_processed",
                Encode!(&item).unwrap(),
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Option<bool>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }

    pub fn set_non_processed_items(&self, values: &[&str]) {
        let values = values.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "set_non_processed_items",
                Encode!(&values).unwrap(),
            )
            .expect("failed to set non-processed items");
        assert_matches!(result, WasmResult::Reply(_));
    }

    pub fn process_single_item_with_panicking_callback(
        &self,
        item: &str,
        future_type: &FutureType,
    ) {
        let res = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "process_single_item_with_panicking_callback",
                Encode!(&item, &future_type).unwrap(),
            )
            .expect_err("process_single_item_with_panicking_callback should panic");
        assert_eq!(res.code, ErrorCode::CanisterCalledTrap);
        assert!(res.description.contains("panicking callback!"));
    }

    pub fn submit_process_single_item_with_panicking_callback(
        &self,
        item: &str,
        future_type: &FutureType,
    ) -> Result<RawMessageId, UserError> {
        self.env.submit_call(
            self.canister_id,
            Principal::anonymous(),
            "process_single_item_with_panicking_callback",
            Encode!(&item, &future_type).unwrap(),
        )
    }

    pub fn process_all_items_with_panicking_callback(
        &self,
        panicking_item: &str,
        future_type: &FutureType,
    ) {
        let res = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "process_all_items_with_panicking_callback",
                Encode!(&panicking_item, &future_type).unwrap(),
            )
            .expect_err("process_all_items_with_panicking_callback should panic");
        assert_eq!(res.code, ErrorCode::CanisterCalledTrap);
        assert!(res.description.contains("panicking callback!"));
    }
}

impl Default for CanisterSetup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(CandidType, Debug, PartialEq, Eq)]
pub enum FutureType {
    TrueAsyncCall,
    FalseAsyncCall,
}
