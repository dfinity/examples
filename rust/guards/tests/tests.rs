use assert_matches::assert_matches;
use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use pocket_ic::PocketIc;

pub const CANISTER_WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/ic_fun_with_guards.wasm");

#[test]
fn should_execute_and_not_revert_guard() {
    let canister = CanisterSetup::new();
    assert_eq!(canister.query_call_get_value(), None);

    canister.update_call_with_panicking_callback(&FutureType::TrueAsyncCall);

    assert_eq!(
        canister.query_call_get_value(),
        Some("guard executed".to_string())
    );
}

#[test]
fn should_revert_guard() {
    let canister = CanisterSetup::new();
    assert_eq!(canister.query_call_get_value(), None);

    canister.update_call_with_panicking_callback(&FutureType::FalseAsyncCall);

    assert_eq!(canister.query_call_get_value(), None);
}

#[test]
fn should_process_elements_only_once() {}

pub struct CanisterSetup {
    env: PocketIc,
    canister_id: CanisterId,
}

impl CanisterSetup {
    pub fn new() -> Self {
        let env = setup_pocket_ic();
        let canister_id = env.create_canister();
        env.add_cycles(canister_id, u128::MAX);
        env.install_canister(canister_id, CANISTER_WASM.to_vec(), vec![], None);
        Self { env, canister_id }
    }

    pub fn query_call_get_value(&self) -> Option<String> {
        use pocket_ic::WasmResult;
        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "get_value",
                Encode!().unwrap(),
            )
            .expect("failed to get value")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Option<String>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get value: {:?}", e);
            }
        }
    }
    pub fn query_call_get_values(&self) -> Vec<String> {
        use pocket_ic::WasmResult;
        match self
            .env
            .query_call(
                self.canister_id,
                Principal::anonymous(),
                "get_values",
                Encode!().unwrap(),
            )
            .expect("failed to get values")
        {
            WasmResult::Reply(bytes) => Decode!(&bytes, Vec<String>).unwrap(),
            WasmResult::Reject(e) => {
                panic!("Failed to get values: {:?}", e);
            }
        }
    }

    pub fn update_call_with_panicking_callback(&self, future_type: &FutureType) {
        use pocket_ic::ErrorCode;

        let res = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "update_with_panicking_callback",
                Encode!(&future_type).unwrap(),
            )
            .expect_err("update_with_panicking_callback should panic");
        assert_eq!(res.code, ErrorCode::CanisterCalledTrap);
        assert!(res.description.contains("panicking callback!"));
    }

    pub fn update_call_set_values(&self, values: &[&str]) {
        let values = values.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let result = self
            .env
            .update_call(
                self.canister_id,
                Principal::anonymous(),
                "update_with_made_up_future_and_panicking_callback",
                Encode!(&values).unwrap(),
            )
            .expect("failed to set values");
        assert_matches!(result, pocket_ic::WasmResult::Reply(_));
    }
}

#[derive(CandidType, Debug, PartialEq, Eq)]
pub enum FutureType {
    TrueAsyncCall,
    FalseAsyncCall,
    ArtificialAsyncCall,
}

fn setup_pocket_ic() -> PocketIc {
    use std::path::PathBuf;
    let filename = match std::env::consts::OS {
        "macos" => "pocket-ic-x86_64-darwin",
        "linux" => "pocket-ic-x86_64-linux",
        _ => panic!("Unsupported OS"),
    };
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR env variable is not defined"),
    );
    let pocket_bin_path = manifest_dir.join("pocket-ic").join(filename);
    std::env::set_var("POCKET_IC_BIN", pocket_bin_path);
    PocketIc::new()
}
