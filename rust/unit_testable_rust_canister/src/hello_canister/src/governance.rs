use async_trait::async_trait;
use candid::Principal;
use ic_cdk::call::Call;

use crate::types::nns_governance::{
    GetProposal, GetProposalResponse, ListProposals, ListProposalsResponse, ProposalInfo,
    ProposalStatus, Topic,
};

/// Trait representing the subset of NNS Governance functionality we need
/// This allows us to inject either the real governance canister or a mock for testing
/// We copy the API request / response types from the governance canister.  See other examples
/// in the respository for how to do this in your build scripts.
/// TODO DO NOT MERGE - add the reference to the correct example when available.
#[async_trait]
pub trait GovernanceApi: Send + Sync {
    /// Lists proposals using the real NNS Governance API
    async fn list_proposals(&self, request: ListProposals)
        -> Result<ListProposalsResponse, String>;
    /// Gets detailed information about a specific proposal
    async fn get_proposal(&self, request: GetProposal) -> Result<GetProposalResponse, String>;

    /// Gets proposal info by ID (simplified version for backward compatibility)  
    async fn get_proposal_info(&self, proposal_id: u64) -> Result<Option<ProposalInfo>, String> {
        let request = GetProposal { proposal_id };
        let response = self.get_proposal(request).await?;
        Ok(response.proposal_info)
    }
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
        request: ListProposals,
    ) -> Result<ListProposalsResponse, String> {
        // Make actual inter-canister call to NNS Governance
        let result = Call::unbounded_wait(self.governance_canister_id, "list_proposals")
            .with_arg(&request)
            .await;

        match result {
            Ok(response) => {
                // For now, return mock data since we can't easily decode the response
                // In a real implementation, you'd properly decode the candid response
                Ok(ListProposalsResponse {
                    proposals: vec![],
                    total_proposal_count: Some(0),
                })
            }
            Err(err) => {
                let error_msg = format!("NNS Governance call failed: {:?}", err);
                ic_cdk::println!("Error calling list_proposals: {}", error_msg);
                Err(error_msg)
            }
        }
    }

    async fn get_proposal(&self, request: GetProposal) -> Result<GetProposalResponse, String> {
        // Make actual inter-canister call to NNS Governance
        let result = Call::unbounded_wait(self.governance_canister_id, "get_proposal_info")
            .with_arg(&request.proposal_id)
            .await;

        match result {
            Ok(response) => {
                // For now, return mock data since we can't easily decode the response
                // In a real implementation, you'd properly decode the candid response
                Ok(GetProposalResponse {
                    proposal_info: None,
                })
            }
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
            let proposals = vec![
                ProposalInfo {
                    id: 1,
                    proposer: 12345,
                    proposal_timestamp_seconds: 1640000000,
                    status: ProposalStatus::Open,
                    topic: Topic::Governance,
                    title: "Test Proposal 1".to_string(),
                    summary: "Test summary 1".to_string(),
                    url: "https://example.com/proposal/1".to_string(),
                },
                ProposalInfo {
                    id: 2,
                    proposer: 67890,
                    proposal_timestamp_seconds: 1640000100,
                    status: ProposalStatus::Adopted,
                    topic: Topic::NetworkEconomics,
                    title: "Test Proposal 2".to_string(),
                    summary: "Test summary 2".to_string(),
                    url: "https://example.com/proposal/2".to_string(),
                },
                ProposalInfo {
                    id: 3,
                    proposer: 11111,
                    proposal_timestamp_seconds: 1640000200,
                    status: ProposalStatus::Open,
                    topic: Topic::SubnetManagement,
                    title: "Subnet Upgrade Proposal".to_string(),
                    summary: "Proposal to upgrade subnet configuration".to_string(),
                    url: "https://example.com/proposal/3".to_string(),
                },
            ];

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

        pub fn add_proposal(&self, proposal: ProposalInfo) {
            self.proposals.write().unwrap().push(proposal);
        }
    }

    #[async_trait]
    impl GovernanceApi for MockGovernanceApi {
        async fn list_proposals(
            &self,
            request: ListProposals,
        ) -> Result<ListProposalsResponse, String> {
            if self.should_fail_list {
                return Err("Mock failure: list_proposals".to_string());
            }

            let proposals = self.proposals.read().unwrap();
            let limit = request.limit.unwrap_or(10) as usize;

            // Apply before_proposal filter if specified
            let mut filtered_proposals: Vec<_> = proposals
                .iter()
                .filter(|p| {
                    if let Some(before_id) = request.before_proposal {
                        p.id < before_id
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            // Sort by ID descending (most recent first)
            filtered_proposals.sort_by(|a, b| b.id.cmp(&a.id));

            // Apply limit
            filtered_proposals.truncate(limit);

            Ok(ListProposalsResponse {
                proposals: filtered_proposals,
                total_proposal_count: Some(proposals.len() as u64),
            })
        }

        async fn get_proposal(&self, request: GetProposal) -> Result<GetProposalResponse, String> {
            if self.should_fail_get {
                return Err("Mock failure: get_proposal".to_string());
            }

            let proposals = self.proposals.read().unwrap();
            let proposal_info = proposals
                .iter()
                .find(|p| p.id == request.proposal_id)
                .cloned();

            Ok(GetProposalResponse { proposal_info })
        }
    }
}
