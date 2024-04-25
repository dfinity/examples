use candid::{Decode, Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use pocket_ic::{PocketIc, WasmResult};

pub const CANISTER_WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/ic_fun_with_guards.wasm");

#[test]
fn should_get_value() {
    let canister = CanisterSetup::new();
    assert_eq!(
        canister.query_call_get_value(),
        Some("Hello, world!".to_string())
    );
}

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
