use std::cell::RefCell;
use std::sync::Arc;
use std::thread::LocalKey;

use crate::governance::GovernanceApi;
use crate::stable_memory;
use crate::types::*;

// =============================================================================
// CANISTER API (main business logic with dependency injection)
// =============================================================================

pub struct CanisterApi {
    governance: Arc<dyn GovernanceApi>,
}

impl CanisterApi {
    /// Constructor that takes the governance dependency
    pub fn new(governance: Arc<dyn GovernanceApi>) -> Self {
        Self { governance }
    }

    /// Gets the current counter value from stable memory
    pub fn get_counter(&self) -> GetCounterResponse {
        let count = stable_memory::get_counter();
        GetCounterResponse { count: Some(count) }
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
        canister_api: &'static LocalKey<RefCell<CanisterApi>>,
    ) -> ListProposalsResponse {
        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            Arc::clone(&api_ref.governance)
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
        proposal_id: Option<u64>,
    ) -> GetProposalInfoResponse {
        let Some(id) = proposal_id else {
            return GetProposalInfoResponse {
                proposal: None,
                error: Some("Missing proposal_id".to_string()),
            };
        };

        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            Arc::clone(&api_ref.governance)
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
        canister_api: &'static LocalKey<RefCell<CanisterApi>>,
    ) -> GetProposalCountResponse {
        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            Arc::clone(&api_ref.governance)
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
        limit: Option<u32>,
    ) -> GetProposalTitlesResponse {
        use crate::types::nns_governance::ListProposals;

        let request = ListProposals {
            limit: Some(limit.unwrap_or(10)),
            omit_large_fields: Some(true), // For performance
            ..Default::default()
        };

        let governance = canister_api.with(|api| {
            let api_ref = api.borrow();
            Arc::clone(&api_ref.governance)
        });

        match governance.list_proposals(request).await {
            Ok(response) => {
                let titles: Vec<String> = response.proposals.into_iter().map(|p| p.title).collect();

                GetProposalTitlesResponse {
                    titles: Some(titles),
                    error: None,
                }
            }
            Err(err) => GetProposalTitlesResponse {
                titles: None,
                error: Some(err),
            },
        }
    }
}

// =============================================================================
// UNIT TESTS (using mocked governance, thread-safe stable memory)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::governance::test_utils::MockGovernanceApi;

    /// Helper to create CanisterApi for testing (not using thread_local)
    use std::cell::RefCell;

    thread_local! {
        static TEST_API: RefCell<CanisterApi> = RefCell::new({
            let governance = Arc::new(MockGovernanceApi::new());
            CanisterApi::new(governance)
        });
    }

    /// Helper to create a direct CanisterApi instance for testing (sync methods)
    fn create_test_api() -> CanisterApi {
        let governance = Arc::new(MockGovernanceApi::new());
        CanisterApi::new(governance)
    }

    /// Helper to create CanisterApi with failure modes for testing
    fn create_test_api_with_failures(list_fail: bool, get_fail: bool) -> CanisterApi {
        let governance = Arc::new(MockGovernanceApi::with_failure_modes(list_fail, get_fail));
        CanisterApi::new(governance)
    }

    #[test]
    fn test_counter_with_stable_memory() {
        // Each test runs in its own thread, so stable memory is isolated
        stable_memory::reset_for_test();

        let governance = Arc::new(MockGovernanceApi::new());
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

    #[tokio::test]
    async fn test_list_proposals_success() {
        // Test successful proposal listing using the mock
        let response = CanisterApi::list_proposals(&TEST_API).await;

        assert!(response.error.is_none());
        assert!(response.proposal_ids.is_some());

        let proposal_ids = response.proposal_ids.unwrap();
        assert_eq!(proposal_ids, vec![3, 2, 1]); // Mock sorts by ID descending (most recent first)
    }

    #[tokio::test]
    async fn test_list_proposals_error() {
        // Create a thread_local API that will fail on list operations
        thread_local! {
            static FAILING_LIST_API: RefCell<CanisterApi> = RefCell::new({
                let governance = Arc::new(MockGovernanceApi::with_failure_modes(true, false));
                CanisterApi::new(governance)
            });
        }

        let response = CanisterApi::list_proposals(&FAILING_LIST_API).await;

        assert!(response.proposal_ids.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Mock failure: list_proposals");
    }

    #[tokio::test]
    async fn test_get_proposal_info_success() {
        // Test successful proposal info retrieval
        let response = CanisterApi::get_proposal_info(&TEST_API, Some(1)).await;

        assert!(response.error.is_none());
        assert!(response.proposal.is_some());

        let proposal = response.proposal.unwrap();
        assert_eq!(proposal.id, 1);
        assert_eq!(proposal.title, "Test Proposal 1");
    }

    #[tokio::test]
    async fn test_get_proposal_info_missing_id() {
        // Test error handling when proposal_id is None
        let response = CanisterApi::get_proposal_info(&TEST_API, None).await;

        assert!(response.proposal.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Missing proposal_id");
    }

    #[tokio::test]
    async fn test_get_proposal_info_not_found() {
        // Test when a proposal ID doesn't exist in mock data (should return None without error)
        let response = CanisterApi::get_proposal_info(&TEST_API, Some(999)).await;

        assert!(response.error.is_none());
        assert!(response.proposal.is_none()); // Valid behavior - proposal not found
    }

    #[tokio::test]
    async fn test_get_proposal_info_error() {
        // Create a thread_local API that will fail on get operations
        thread_local! {
            static FAILING_GET_API: RefCell<CanisterApi> = RefCell::new({
                let governance = Arc::new(MockGovernanceApi::with_failure_modes(false, true));
                CanisterApi::new(governance)
            });
        }

        let response = CanisterApi::get_proposal_info(&FAILING_GET_API, Some(1)).await;

        assert!(response.proposal.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Mock failure: get_proposal");
    }
}
