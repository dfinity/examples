use candid::Principal;
use ic_cdk::call::Call;
use ic_cdk::query;
use ic_cdk::update;
use ic_cdk_management_canister::{
    CanisterInstallMode, CanisterSettings, CreateCanisterArgs, InstallCodeArgs,
    create_canister_with_extra_cycles, install_code,
};

use std::sync::Arc;
use std::sync::RwLock;

const NUM_PARTITIONS: usize = 5;

// Bucket WASM compiled separately and embedded at compile time via include_bytes!;
// the backend installs it into new canister instances at runtime on first put().
pub const WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/bucket.wasm");

thread_local! {
    // A list of canister IDs for data partitions
    static CANISTER_IDS: Arc<RwLock<Vec<Principal>>> = Arc::new(RwLock::new(vec![]));
}

#[update]
async fn put(key: u128, value: u128) -> Option<u128> {
    // Create partitions if they don't exist yet
    if CANISTER_IDS.with(|canister_ids| canister_ids.read().unwrap().is_empty()) {
        for _ in 0..NUM_PARTITIONS {
            create_partition_canister().await;
        }
    }

    let canister_id = get_partition_for_key(key);
    ic_cdk::println!(
        "Put in caller for key={} .. using callee={}",
        key,
        canister_id.to_text()
    );

    Call::bounded_wait(canister_id, "put")
        .with_args(&(key, value))
        .await
        .ok()
        .and_then(|r| r.candid_tuple::<(Option<u128>,)>().ok())
        .and_then(|(v,)| v)
}

#[query(composite = true)]
async fn get(key: u128) -> Option<u128> {
    let canister_id = get_partition_for_key(key);
    ic_cdk::println!(
        "Get in caller for key={} .. using callee={}",
        key,
        canister_id.to_text()
    );

    Call::bounded_wait(canister_id, "get")
        .with_arg(key)
        .await
        .ok()
        .and_then(|r| r.candid_tuple::<(Option<u128>,)>().ok())
        .and_then(|(v,)| v)
}

#[update]
async fn get_update(key: u128) -> Option<u128> {
    let canister_id = get_partition_for_key(key);
    ic_cdk::println!(
        "Get as update in caller for key={} .. using callee={}",
        key,
        canister_id.to_text()
    );

    Call::bounded_wait(canister_id, "get")
        .with_arg(key)
        .await
        .ok()
        .and_then(|r| r.candid_tuple::<(Option<u128>,)>().ok())
        .and_then(|(v,)| v)
}

fn get_partition_for_key(key: u128) -> Principal {
    CANISTER_IDS.with(|canister_ids| {
        let canister_ids = canister_ids.read().unwrap();
        canister_ids[lookup(key).0 as usize]
    })
}

#[query(composite = true)]
fn lookup(key: u128) -> (u128, String) {
    let r = key % NUM_PARTITIONS as u128;
    (
        r,
        CANISTER_IDS.with(|canister_ids| {
            let canister_ids = canister_ids.read().unwrap();
            canister_ids[r as usize].to_text()
        }),
    )
}

ic_cdk::export_candid!();

async fn create_partition_canister() {
    // Divide the available balance equally among all buckets plus the backend itself,
    // mirroring the Motoko approach: cycleShare = Cycles.balance() / (n + 1).
    let cycle_share =
        ic_cdk::api::canister_cycle_balance() / (NUM_PARTITIONS as u128 + 1);

    let create_args = CreateCanisterArgs {
        settings: Some(CanisterSettings {
            controllers: Some(vec![ic_cdk::api::canister_self()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            log_memory_limit: None,
            wasm_memory_limit: None,
            wasm_memory_threshold: None,
            environment_variables: None,
        }),
    };

    let result = create_canister_with_extra_cycles(&create_args, cycle_share)
        .await
        .unwrap();
    let canister_id = result.canister_id;

    ic_cdk::println!("Created bucket canister {}", canister_id);

    let install_args = InstallCodeArgs {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: WASM.to_vec(),
        arg: vec![],
    };

    install_code(&install_args).await.unwrap();

    CANISTER_IDS.with(|canister_ids| {
        canister_ids.write().unwrap().push(canister_id);
    });
}
