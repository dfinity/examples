use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::sync::Arc;

mod canister_api;
mod counter;
mod governance;
mod stable_memory;
pub mod types;

use crate::counter::StableMemoryCounter;
use canister_api::CanisterApi;
use governance::NnsGovernanceApi;
use types::*;

thread_local! {
    // Canister API instance with production dependencies
    // This uses dependency injection to make unit-testing easier.
    pub static CANISTER_API: RefCell<CanisterApi> = RefCell::new({
        let governance = Arc::new(NnsGovernanceApi::new());
        let counter = Arc::new(StableMemoryCounter);
        CanisterApi::new(governance, counter)
    });
}

// =============================================================================
// IC CANISTER ENDPOINTS (Request/Response pattern even with no arguments allows for API evolution)
// =============================================================================

#[init]
fn init() {
    ic_cdk::println!("Canister initialized");
}

#[pre_upgrade]
fn pre_upgrade() {}

#[post_upgrade]
fn post_upgrade() {}

#[query]
fn get_count(_request: GetCountRequest) -> GetCountResponse {
    CANISTER_API.with(|api| api.borrow().get_count())
}

#[update]
fn increment_count(_request: IncrementCountRequest) -> IncrementCountResponse {
    CANISTER_API.with(|api| api.borrow().increment_count())
}

#[update]
fn decrement_count(_request: DecrementCountRequest) -> DecrementCountResponse {
    CANISTER_API.with(|api| api.borrow().decrement_count())
}

#[update]
async fn get_proposal_info(request: GetProposalInfoRequest) -> GetProposalInfoResponse {
    CanisterApi::get_proposal_info(&CANISTER_API, request).await
}

#[update]
async fn get_proposal_titles(request: GetProposalTitlesRequest) -> GetProposalTitlesResponse {
    CanisterApi::get_proposal_titles(&CANISTER_API, request).await
}

// Export candid interface
ic_cdk::export_candid!();

#[cfg(test)]
mod tests {
    use super::*;
    use candid_parser::utils::{service_equal, CandidSource};
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn candid_interface_compatibility() {
        // Get the directory where this crate's Cargo.toml is located
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not set");
        let candid_file_path = PathBuf::from(&manifest_dir).join("hello_canister.did");

        // Read the declared interface from the .did file
        let declared_interface_str =
            std::fs::read_to_string(&candid_file_path).unwrap_or_else(|e| {
                panic!(
                    "Failed to read candid file at {:?}: {}",
                    candid_file_path, e
                )
            });

        // Get the actual interface from the implementation using export_candid
        let actual_interface_str = __export_service();

        // Normalize both interfaces for comparison (remove extra whitespace, etc.)
        let declared_interface = CandidSource::Text(&declared_interface_str);
        let actual_interface = CandidSource::Text(&actual_interface_str);

        let result = service_equal(declared_interface, actual_interface);
        assert!(result.is_ok(), "{:?}\n\n", result);
    }
}
