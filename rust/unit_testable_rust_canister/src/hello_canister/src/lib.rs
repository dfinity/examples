use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::sync::Arc;

mod canister_api;
mod governance;
mod stable_memory;
pub mod types;

use canister_api::CanisterApi;
use governance::NnsGovernanceApi;
use types::*;

thread_local! {
    /// Canister API instance with production dependencies
    /// Following SNS-WASM pattern where CanisterApi is stored in thread_local
    pub static CANISTER_API: RefCell<CanisterApi> = RefCell::new({
        let governance = Arc::new(NnsGovernanceApi::new());
        CanisterApi::new(governance)
    });
}

// =============================================================================
// IC CANISTER ENDPOINTS (Request/Response pattern for API evolution)
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
fn greet(request: GreetRequest) -> GreetResponse {
    CANISTER_API.with(|api| api.borrow().greet(request.name))
}

#[query]
fn get_counter(_request: GetCounterRequest) -> GetCounterResponse {
    CANISTER_API.with(|api| api.borrow().get_counter())
}

#[update]
fn increment_counter(_request: IncrementCounterRequest) -> IncrementCounterResponse {
    CANISTER_API.with(|api| api.borrow().increment_counter())
}

#[update]
async fn list_proposals(_request: ListProposalsRequest) -> ListProposalsResponse {
    CanisterApi::list_proposals(&CANISTER_API).await
}

#[update]
async fn get_proposal_info(request: GetProposalInfoRequest) -> GetProposalInfoResponse {
    CanisterApi::get_proposal_info(&CANISTER_API, request.proposal_id).await
}

#[update]
async fn get_proposal_count(_request: GetProposalCountRequest) -> GetProposalCountResponse {
    CanisterApi::get_proposal_count(&CANISTER_API).await
}

#[update]
async fn get_proposal_titles(request: GetProposalTitlesRequest) -> GetProposalTitlesResponse {
    CanisterApi::get_proposal_titles(&CANISTER_API, request.limit).await
}

// Export candid interface
ic_cdk::export_candid!();
