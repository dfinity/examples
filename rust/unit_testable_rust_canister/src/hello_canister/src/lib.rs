
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::thread::LocalKey;

mod governance;
mod stable_memory;
pub mod types;

use governance::{GovernanceApi, GovernanceApiWrapper};
use types::*;

// =============================================================================
// CANISTER API (main business logic with dependency injection)
// =============================================================================

pub struct CanisterApi {
    governance: GovernanceApiWrapper,
}

impl CanisterApi {
    /// Constructor that takes the governance dependency
    pub fn new(governance: GovernanceApiWrapper) -> Self {
        Self { governance }
    }

    /// Handles greeting functionality (normal &self method for sync operations)
    pub fn greet(&self, name: Option<String>) -> GreetResponse {
        let greeting = match name {
            Some(n) if !n.is_empty() => {
                format!("Hello, {}! Welcome to the Internet Computer!", n)
            }
            _ => "Hello, Anonymous!".to_string(),
        };
        
        GreetResponse {
            greeting: Some(greeting),
        }
    }

    /// Gets the current counter value from stable memory
    pub fn get_counter(&self) -> GetCounterResponse {
        let count = stable_memory::get_counter();
        GetCounterResponse {
            count: Some(count),
        }
    }

    /// Increments the counter in stable memory and returns new value
    pub fn increment_counter(&self) -> IncrementCounterResponse {
        let new_count = stable_memory::increment_counter();
        IncrementCounterResponse {
            new_count: Some(new_count),
        }
    }

    /// SNS-WASM Pattern: Static method for canister endpoints that takes LocalKey
    /// Lists all proposals from NNS Governance
    pub async fn list_proposals(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>
    ) -> ListProposalsResponse {
        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            api_ref.governance.clone()
        });

        match governance.list_proposal_ids().await {
            Ok(proposal_ids) => ListProposalsResponse {
                proposal_ids: Some(proposal_ids),
                error: None,
            },
            Err(err) => ListProposalsResponse {
                proposal_ids: None,
                error: Some(err),
            },
        }
    }

    /// Static method for canister endpoints - Gets proposal information by ID
    pub async fn get_proposal_info(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>, 
        proposal_id: Option<u64>
    ) -> GetProposalInfoResponse {
        let Some(id) = proposal_id else {
            return GetProposalInfoResponse {
                proposal: None,
                error: Some("Missing proposal_id".to_string()),
            };
        };

        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            api_ref.governance.clone()
        });

        match governance.get_proposal_info(id).await {
            Ok(proposal) => GetProposalInfoResponse {
                proposal,
                error: None,
            },
            Err(err) => GetProposalInfoResponse {
                proposal: None,
                error: Some(err),
            },
        }
    }

    /// Static method for canister endpoints - Gets proposal count
    pub async fn get_proposal_count(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>
    ) -> GetProposalCountResponse {
        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            api_ref.governance.clone()
        });

        match governance.list_proposal_ids().await {
            Ok(proposals) => GetProposalCountResponse {
                count: Some(proposals.len()),
                error: None,
            },
            Err(err) => GetProposalCountResponse {
                count: None,
                error: Some(err),
            },
        }
    }

    /// Static method for canister endpoints - Gets the latest proposal titles
    pub async fn get_proposal_titles(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>, 
        limit: Option<u32>
    ) -> GetProposalTitlesResponse {
        use crate::types::nns_governance::ListProposals;
        
        let request = ListProposals {
            limit: Some(limit.unwrap_or(10)),
            omit_large_fields: Some(true), // For performance
            ..Default::default()
        };

        let governance = canister_api.with_borrow(|api| {
            api.governance.clone()
        });

        match governance.list_proposals(request).await {
            Ok(response) => {
                let titles: Vec<String> = response.proposals
                    .into_iter()
                    .map(|p| p.title)
                    .collect();
                
                GetProposalTitlesResponse {
                    titles: Some(titles),
                    error: None,
                }
            },
            Err(err) => GetProposalTitlesResponse {
                titles: None,
                error: Some(err),
            },
        }
    }

}

// =============================================================================
// THREAD_LOCAL CANISTER API (SNS-WASM pattern)
// =============================================================================

thread_local! {
    /// Canister API instance with production dependencies
    /// Following SNS-WASM pattern where CanisterApi is stored in thread_local
    static CANISTER_API: RefCell<CanisterApi> = RefCell::new({
        let governance = GovernanceApiWrapper::production();
        CanisterApi::new(governance)
    });
}

// SNS-WASM Pattern: LocalKey is passed directly to static methods
// No helper functions needed - methods access thread-local state via LocalKey parameter

// =============================================================================
// IC CANISTER ENDPOINTS (Request/Response pattern for API evolution)
// =============================================================================

#[init]
fn init() {
    ic_cdk::println!("Canister initialized");
}

#[pre_upgrade]
fn pre_upgrade() {

}

#[post_upgrade]
fn post_upgrade() {

}

#[query]
fn greet(request: GreetRequest) -> GreetResponse {
    CANISTER_API.with(|api| {
        api.borrow().greet(request.name)
    })
}

#[query] 
fn get_counter(_request: GetCounterRequest) -> GetCounterResponse {
    CANISTER_API.with(|api| {
        api.borrow().get_counter()
    })
}

#[update]
fn increment_counter(_request: IncrementCounterRequest) -> IncrementCounterResponse {
    CANISTER_API.with(|api| {
        api.borrow().increment_counter()
    })
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

// =============================================================================
// UNIT TESTS (using mocked governance, thread-safe stable memory)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use governance::test_utils::MockGovernanceApi;

    /// Helper to create CanisterApi for testing (not using thread_local)
    use std::cell::RefCell;

    thread_local! {
        static TEST_API: RefCell<CanisterApi> = RefCell::new({
            let governance = GovernanceApiWrapper::mock();
            CanisterApi::new(governance)
        });
    }

    /// Helper to reset test API with fresh mock data
    fn reset_test_api() {
        TEST_API.with(|api| {
            let governance = GovernanceApiWrapper::mock();
            *api.borrow_mut() = CanisterApi::new(governance);
        });
    }

    /// Helper to create test API with failure modes
    fn reset_test_api_with_failures(list_fail: bool, get_fail: bool) {
        TEST_API.with(|api| {
            let governance = GovernanceApiWrapper::mock_with_failures(list_fail, get_fail);
            *api.borrow_mut() = CanisterApi::new(governance);
        });
    }

    /// Helper to create a direct CanisterApi instance for testing (sync methods)
    fn create_test_api() -> CanisterApi {
        let governance = GovernanceApiWrapper::mock();
        CanisterApi::new(governance)
    }

    /// Helper to create CanisterApi with failure modes for testing
    fn create_test_api_with_failures(list_fail: bool, get_fail: bool) -> CanisterApi {
        let governance = GovernanceApiWrapper::mock_with_failures(list_fail, get_fail);
        CanisterApi::new(governance)
    }

    #[test]
    fn test_greet_request_response() {
        let governance = GovernanceApiWrapper::mock();
        let api = CanisterApi::new(governance);
        
        let response = api.greet(Some("Alice".to_string()));
        assert_eq!(response.greeting, Some("Hello, Alice! Welcome to the Internet Computer!".to_string()));
        
        let response = api.greet(Some("".to_string()));
        assert_eq!(response.greeting, Some("Hello, Anonymous!".to_string()));
        
        let response = api.greet(None);
        assert_eq!(response.greeting, Some("Hello, Anonymous!".to_string()));
    }

    #[test]
    fn test_counter_with_stable_memory() {
        // Each test runs in its own thread, so stable memory is isolated
        stable_memory::reset_for_test();
        
        let governance = GovernanceApiWrapper::mock();
        let api = CanisterApi::new(governance);
        
        let response = api.get_counter();
        assert_eq!(response.count, Some(0));
        
        let response = api.increment_counter();
        assert_eq!(response.new_count, Some(1));
        
        let response = api.increment_counter();
        assert_eq!(response.new_count, Some(2));
        
        let response = api.get_counter();
        assert_eq!(response.count, Some(2));
    }

    #[test]
    fn test_thread_isolation_of_stable_memory() {
        // This test demonstrates that stable memory is thread-local
        // Each test gets its own isolated state
        stable_memory::reset_for_test();
        
        let api = create_test_api();
        
        // Set some state
        api.increment_counter();
        api.increment_counter();
        api.increment_counter();
        
        let response = api.get_counter();
        assert_eq!(response.count, Some(3));
        
        // Other tests won't see this state because they run in different threads
        // and thread_local storage is isolated per thread
    }

    #[test]
    fn test_request_response_evolution() {
        // This test demonstrates how the Request/Response pattern supports API evolution
        let api = create_test_api();
        
        // Future fields can be added to requests as Optional without breaking existing clients
        let greet_request = GreetRequest {
            name: Some("Evolution Test".to_string()),
            // future_field: None, // Can be added later without breaking changes
        };
        
        let response = api.greet(greet_request.name);
        
        // Response can also evolve by adding optional fields
        assert!(response.greeting.is_some());
        // assert!(response.future_response_field.is_none()); // Can be added later
    }
}

// Export candid interface
ic_cdk::export_candid!();
