use crate::counter::Counter;
use crate::governance::GovernanceApi;
use crate::stable_memory;
use crate::types::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread::LocalKey;

// =============================================================================
// CANISTER API (main business logic)
// =============================================================================

pub struct CanisterApi {
    // Note: This can also be done as generic on CanisterApi, but when that pattern can get unwieldy
    // when you have many dependencies.  And you would still need to put it in an Arc in order
    // to be able to clone the API out of the thread_local.  The advantage of doing this, however,
    // is that it makes it possible to have stateful dependencies that can be easily mocked,
    governance: Arc<dyn GovernanceApi>,

    counter: Arc<dyn Counter>,
}

impl CanisterApi {
    /// Constructor that takes the governance dependency
    pub fn new(governance: Arc<dyn GovernanceApi>, counter: Arc<dyn Counter>) -> Self {
        Self {
            governance,
            counter,
        }
    }

    /// Gets the current counter value from stable memory
    pub fn get_count(&self) -> GetCountResponse {
        let count = self.counter.get_count();
        GetCountResponse { count: Some(count) }
    }

    /// Increments the counter in stable memory and returns new value
    pub fn increment_count(&self) -> IncrementCountResponse {
        let new_count = self.counter.increment_count();
        IncrementCountResponse {
            new_count: Some(new_count),
        }
    }

    pub fn decrement_count(&self) -> DecrementCountResponse {
        let new_count = self.counter.decrement_count();
        DecrementCountResponse {
            new_count: Some(new_count),
        }
    }

    /// Helper method to extract governance API from thread_local CanisterApi for use in async methods.
    fn get_governance(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>,
    ) -> Arc<dyn GovernanceApi> {
        canister_api.with(|api| {
            let api_ref = api.borrow();
            Arc::clone(&api_ref.governance)
        })
    }

    /// Static method for canister endpoints - Gets proposal information by ID
    pub async fn get_proposal_info(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>,
        request: GetProposalInfoRequest,
    ) -> GetProposalInfoResponse {
        let Some(id) = request.proposal_id else {
            return GetProposalInfoResponse {
                proposal: None,
                error: Some("Missing proposal_id".to_string()),
            };
        };

        let governance = Self::get_governance(canister_api);

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

    /// Static method for canister endpoints - Gets the latest proposal titles
    pub async fn get_proposal_titles(
        canister_api: &'static LocalKey<RefCell<CanisterApi>>,
        request: GetProposalTitlesRequest,
    ) -> GetProposalTitlesResponse {
        use crate::types::nns_governance::ListProposalInfo;

        let request = ListProposalInfo {
            limit: request.limit.unwrap_or(10),
            omit_large_fields: Some(true), // For performance
            ..Default::default()
        };

        let governance = Self::get_governance(canister_api);

        match governance.list_proposals(request).await {
            Ok(response) => {
                let titles: Vec<String> = response
                    .proposal_info
                    .into_iter()
                    .map(|p| p.proposal.unwrap().title.unwrap())
                    .collect();

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

    use crate::counter::test_util::TestCounter;
    use crate::types::nns_governance::Canister;
    /// Helper to create CanisterApi for testing (not using thread_local)
    use std::cell::RefCell;

    thread_local! {
        static TEST_API: RefCell<CanisterApi> = RefCell::new({
            let governance = Arc::new(MockGovernanceApi::new());
            let counter = Arc::new(TestCounter::new());
            CanisterApi::new(governance, counter)
        });
    }

    /// Helper to create a direct CanisterApi instance for testing (sync methods)
    fn create_test_api() -> CanisterApi {
        let governance = Arc::new(MockGovernanceApi::new());
        let counter = Arc::new(TestCounter::new());
        CanisterApi::new(governance, counter)
    }

    #[test]
    fn test_counter_endpoints() {
        // Each test runs in its own thread, so stable memory is isolated
        let api = create_test_api();

        let response = api.get_count();
        assert_eq!(response.count, Some(0));

        let response = api.increment_count();
        assert_eq!(response.new_count, Some(1));

        let response = api.increment_count();
        assert_eq!(response.new_count, Some(2));

        let response = api.get_count();
        assert_eq!(response.count, Some(2));

        let response = api.decrement_count();
        assert_eq!(response.new_count, Some(1));

        let response = api.get_count();
        assert_eq!(response.count, Some(1));

        // test that it can't underflow.
        let response = api.decrement_count();
        let response = api.decrement_count();
        let response = api.decrement_count();
        assert_eq!(response.new_count, Some(0));
    }
    #[tokio::test]
    async fn test_get_proposal_info_success() {
        // Test successful proposal info retrieval
        let response = CanisterApi::get_proposal_info(
            &TEST_API,
            GetProposalInfoRequest {
                proposal_id: Some(1),
            },
        )
        .await;

        assert!(response.error.is_none());
        assert!(response.proposal.is_some());

        let proposal = response.proposal.unwrap();
        assert_eq!(proposal.id.unwrap().id, 1);
        assert_eq!(
            proposal.proposal.unwrap().title,
            Some("Test title 1".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_proposal_info_missing_id() {
        // Test error handling when proposal_id is None
        let response =
            CanisterApi::get_proposal_info(&TEST_API, GetProposalInfoRequest { proposal_id: None })
                .await;

        assert!(response.proposal.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Missing proposal_id");
    }

    #[tokio::test]
    async fn test_get_proposal_info_not_found() {
        // Test when a proposal ID doesn't exist in mock data (should return None without error)
        let response = CanisterApi::get_proposal_info(
            &TEST_API,
            GetProposalInfoRequest {
                proposal_id: Some(999),
            },
        )
        .await;

        assert!(response.error.is_none());
        assert!(response.proposal.is_none()); // Valid behavior - proposal not found
    }

    #[tokio::test]
    async fn test_get_proposal_info_error() {
        // Create a thread_local API that will fail on get operations
        thread_local! {
            static FAILING_GET_API: RefCell<CanisterApi> = RefCell::new({
                let governance = Arc::new(MockGovernanceApi::with_failure_modes(false, true));
                let test_counter = Arc::new(TestCounter::new());
                CanisterApi::new(governance, test_counter)
            });
        }

        let response = CanisterApi::get_proposal_info(
            &FAILING_GET_API,
            GetProposalInfoRequest {
                proposal_id: Some(1),
            },
        )
        .await;

        assert!(response.proposal.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap(), "Mock failure: get_proposal");
    }

    // Note, see the integration test.  Notice the difference in runtime between the two tests, despite
    // the fact that the setup and tests do largely the same thing.  For complex test scenarios,
    // testing logic is always going to be much faster, which makes this style of testing preferred
    // to allow for faster development (quicker feedback loops) and more thorough testing able to
    // be performed quickly on each change.
    #[tokio::test]
    async fn test_get_proposal_titles() {
        async fn test_limits(limit: Option<u32>) {
            let response = CanisterApi::get_proposal_titles(
                &TEST_API,
                GetProposalTitlesRequest {
                    limit: limit.clone(),
                },
            )
            .await;
            assert_eq!(response.titles.unwrap().len() as u32, limit.unwrap_or(10));
        }

        test_limits(None).await;
        test_limits(Some(20)).await;
        test_limits(Some(1)).await;
    }
}
