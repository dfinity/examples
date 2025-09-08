use crate::types::nns_governance::{ListProposalInfo, ListProposalInfoResponse, ProposalInfo};
use async_trait::async_trait;
use candid::Principal;
use ic_cdk::call::Call;

/// Trait representing the subset of NNS Governance functionality we need
/// This allows us to inject either the real governance canister or a mock for testing
/// We copy the API request / response types from the governance canister.  See other examples
/// in the respository for how to do this in your build scripts.
/// TODO DO NOT MERGE - add the reference to the correct example when available.
#[async_trait]
pub trait GovernanceApi: Send + Sync {
    /// Lists proposals using the real NNS Governance API
    async fn list_proposals(
        &self,
        request: ListProposalInfo,
    ) -> Result<ListProposalInfoResponse, String>;
    /// Gets detailed information about a specific proposal
    async fn get_proposal_info(&self, proposal_id: u64) -> Result<Option<ProposalInfo>, String>;
}

/// Production implementation that makes actual inter-canister calls to NNS Governance
#[derive(Clone)]
pub struct NnsGovernanceApi {
    /// The principal of the NNS Governance canister
    governance_canister_id: Principal,
}

impl NnsGovernanceApi {
    /// Creates a new NnsGovernanceApi with the real governance canister ID
    pub fn new() -> Self {
        // This is the actual NNS Governance canister ID on mainnet
        let governance_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
            .expect("Invalid NNS Governance canister ID");

        Self {
            governance_canister_id,
        }
    }

    /// Creates a new NnsGovernanceApi with a custom governance canister ID (for testing)
    #[allow(dead_code)]
    pub fn with_canister_id(governance_canister_id: Principal) -> Self {
        Self {
            governance_canister_id,
        }
    }
}

impl Default for NnsGovernanceApi {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GovernanceApi for NnsGovernanceApi {
    async fn list_proposals(
        &self,
        request: ListProposalInfo,
    ) -> Result<ListProposalInfoResponse, String> {
        // Make actual inter-canister call to NNS Governance
        let result = Call::unbounded_wait(self.governance_canister_id, "list_proposals")
            .with_arg(&request)
            .await;

        match result {
            Ok(response) => response
                .candid()
                .map_err(|e| format!("Could not decode candid: {:?}", e)),
            Err(err) => {
                let error_msg = format!("NNS Governance call failed: {:?}", err);
                ic_cdk::println!("Error calling list_proposals: {}", error_msg);
                Err(error_msg)
            }
        }
    }

    async fn get_proposal_info(&self, proposal_id: u64) -> Result<Option<ProposalInfo>, String> {
        // Make actual inter-canister call to NNS Governance
        let result = Call::unbounded_wait(self.governance_canister_id, "get_proposal_info")
            .with_arg(proposal_id)
            .await;

        match result {
            Ok(response) => response
                .candid()
                .map_err(|e| format!("Could not decode candid: {:?}", e)),
            Err(err) => {
                let error_msg = format!("NNS Governance call failed: {:?}", err);
                ic_cdk::println!("Error calling get_proposal_info: {}", error_msg);
                Err(error_msg)
            }
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::types::nns_governance::{NeuronId, Proposal, ProposalId};
    use std::sync::{Arc, RwLock};

    /// Mock implementation for testing
    #[derive(Clone)]
    pub struct MockGovernanceApi {
        pub proposals: Arc<RwLock<Vec<ProposalInfo>>>,
        pub should_fail_list: bool,
        pub should_fail_get: bool,
    }

    impl MockGovernanceApi {
        pub fn new() -> Self {
            let proposals = (0..20)
                .map(|id| ProposalInfo {
                    id: Some(ProposalId { id }),
                    proposer: Some(NeuronId { id: 123 }),
                    proposal_timestamp_seconds: 1640000000,
                    reward_event_round: 0,
                    deadline_timestamp_seconds: None,
                    failed_timestamp_seconds: 0,
                    reject_cost_e8s: 0,
                    derived_proposal_information: None,
                    latest_tally: None,
                    total_potential_voting_power: None,
                    reward_status: 0,
                    decided_timestamp_seconds: 0,
                    status: 1,
                    topic: 13,
                    failure_reason: None,
                    ballots: vec![],
                    proposal: Some(Box::from(Proposal {
                        url: "".to_string(),
                        title: Some(format!("Test title {id}")),
                        action: None,
                        summary: "".to_string(),
                    })),
                    executed_timestamp_seconds: 0,
                })
                .collect();

            Self {
                proposals: Arc::new(RwLock::new(proposals)),
                should_fail_list: false,
                should_fail_get: false,
            }
        }

        pub fn with_failure_modes(should_fail_list: bool, should_fail_get: bool) -> Self {
            let mut mock = Self::new();
            mock.should_fail_list = should_fail_list;
            mock.should_fail_get = should_fail_get;
            mock
        }
    }

    #[async_trait]
    impl GovernanceApi for MockGovernanceApi {
        async fn list_proposals(
            &self,
            request: ListProposalInfo,
        ) -> Result<ListProposalInfoResponse, String> {
            if self.should_fail_list {
                return Err("Mock failure: list_proposals".to_string());
            }

            let proposals = self.proposals.read().unwrap();
            let limit = request.limit as usize;

            let before_id = request
                .before_proposal
                .unwrap_or(ProposalId { id: u64::MAX })
                .id;
            // Apply before_proposal filter if specified
            let mut filtered_proposals: Vec<_> = proposals
                .iter()
                .filter(|p| p.id.as_ref().unwrap().id < before_id)
                .cloned()
                .collect();

            // Apply limit
            filtered_proposals.truncate(limit);

            Ok(ListProposalInfoResponse {
                proposal_info: filtered_proposals,
            })
        }

        async fn get_proposal_info(
            &self,
            proposal_id: u64,
        ) -> Result<Option<ProposalInfo>, String> {
            if self.should_fail_get {
                return Err("Mock failure: get_proposal".to_string());
            }

            let proposals = self.proposals.read().unwrap();
            let proposal_info = proposals
                .iter()
                .find(|p| p.id.as_ref().unwrap().id == proposal_id)
                .cloned();

            Ok(proposal_info)
        }
    }
}
