use ic_cdk::api::call::{call};
use ic_cdk::api::management_canister::main::{CreateCanisterArgument, create_canister, InstallCodeArgument, install_code, CanisterInstallMode};
use ic_cdk::api::management_canister::provisional::CanisterSettings;
use ic_cdk_macros::{query, update, init};
use candid::Principal;

use std::sync::Arc;
use std::sync::RwLock;

const NUM_PARTITIONS: usize = 5;

// Inline wasm binary of data partition canister
pub const WASM: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/data_partition.wasm");

thread_local! {
    // A list of canister IDs for data partitions
    static CANISTER_IDS: Arc<RwLock<Vec<Principal>>> = Arc::new(RwLock::new(vec![]));
}
#[update]
async fn put(key: u128, value: u128) -> Option<u128> {
    let canister_id = get_partition_for_key(key);
    match call(canister_id, "put", (key, value), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

#[query(composite = true)]
async fn get(key: u128) -> Option<u128> {
    let canister_id = get_partition_for_key(key);
    match call(canister_id, "get", (key, ), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

#[update]
async fn get_update(key: u128) -> Option<u128> {
    let canister_id = get_partition_for_key(key);
    match call(canister_id, "get", (key, ), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

fn get_partition_for_key(key: u128) -> Principal {
    let canister_id = CANISTER_IDS.with(|canister_id| {
        let canister_id = canister_id.read().unwrap();
        canister_id[lookup(key) as usize]
    });
    canister_id
}

#[query(composite = true)]
fn lookup(key: u128) -> u128 {
    key % NUM_PARTITIONS as u128
}

async fn create_data_partition_canister_from_wasm() {
    let create_args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(vec![ic_cdk::id()]),
            compute_allocation: Some(0.into()),
            memory_allocation: Some(0.into()),
            freezing_threshold: Some(0.into()),
        })
    };

    let canister_record = create_canister(create_args).await.unwrap();
    let canister_id = canister_record.0.canister_id;

    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: WASM.to_vec(),
        arg: vec![],
    };

    install_code(install_args).await.unwrap();

    CANISTER_IDS.with(|canister_ids| {
        let mut canister_ids = canister_ids.write().unwrap();
        canister_ids.push(canister_id);
    });
}

#[update]
async fn init() {
    ic_cdk::print("Initializing kv_frontend");
    for _ in 0..NUM_PARTITIONS {
        create_data_partition_canister_from_wasm().await;
    }
}