use ic_cdk::api::call::{call};
use ic_cdk_macros::{query, update};
use candid::Principal;

const NUM_PARTITIONS: usize = 1;

// Add canister IDs later
// Maybe later, we could add partitions dynamically from the kv-frontend canister
// We could add an explicit method to add more partitions.
const CANISTER_IDS: [&str; NUM_PARTITIONS] = ["bkyz2-fmaaa-aaaaa-qaaaq-cai"];

#[update]
async fn put(key: u128, value: u128) -> Option<u128> {
    let p: Principal = Principal::from_text(CANISTER_IDS[lookup(key) as usize]).unwrap();
    match call(p, "put", (key, value), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

#[query(composite = true)]
async fn get(key: u128) -> Option<u128> {
    let p: Principal = Principal::from_text(CANISTER_IDS[lookup(key) as usize]).unwrap();
    match call(p, "get", (key, ), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

#[update]
async fn get_update(key: u128) -> Option<u128> {
    let p: Principal = Principal::from_text(CANISTER_IDS[lookup(key) as usize]).unwrap();
    match call(p, "get", (key, ), ).await {
        Ok(r) => {
            let (res,): (Option<u128>,) = r;
            res
        },
        Err(_) => None,
    }
}

#[query(composite = true)]
fn lookup(key: u128) -> u128 {
    key % NUM_PARTITIONS as u128
}
