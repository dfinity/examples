use ic_cdk::api::management_canister::{main::canister_status, provisional::CanisterIdRecord};

#[ic_cdk::update]
// Returns the current query stats as a string as received from the canister itself via the canister status.
async fn get_query_stats() -> String {
    let query_stats = canister_status(CanisterIdRecord {
        canister_id: ic_cdk::id(),
    })
    .await
    .unwrap()
    .0
    .query_stats;

    format!(
        "Number of calls: {} - Number of instructions {} - Request payload bytes: {} - Response payload bytes: {}", 
        query_stats.num_calls_total,
        query_stats.num_instructions_total,
        query_stats.request_payload_bytes_total,
        query_stats.response_payload_bytes_total
    )
}

#[ic_cdk::query]
// Used to generate load to the system. Uses time() in order to prevent caching.
// This is mostly to test query stats and is being called periodically by a load generator.
fn load() -> u64 {
    ic_cdk::api::time()
}
