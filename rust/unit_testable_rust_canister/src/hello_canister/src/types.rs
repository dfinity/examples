use candid::{CandidType, Deserialize};
use serde::{Serialize};

// =============================================================================
// REQUEST TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GreetRequest {
    pub name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetCounterRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IncrementCounterRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ListProposalsRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalInfoRequest {
    pub proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GreetWithProposalRequest {
    pub name: Option<String>,
    pub proposal_id: Option<u64>,
}

// =============================================================================
// RESPONSE TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GreetResponse {
    pub greeting: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetCounterResponse {
    pub count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IncrementCounterResponse {
    pub new_count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ListProposalsResponse {
    pub proposal_ids: Option<Vec<u64>>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalInfoResponse {
    pub proposal: Option<nns_governance::ProposalInfo>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalCountResponse {
    pub count: Option<usize>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GreetWithProposalResponse {
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalTitlesRequest {
    pub limit: Option<u32>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalTitlesResponse {
    pub titles: Option<Vec<String>>,
    pub error: Option<String>,
}

// =============================================================================
// NNS GOVERNANCE TYPES (from IC monorepo patterns)
// =============================================================================

pub mod nns_governance {
    use candid::{CandidType, Deserialize};
    use serde::{ Serialize};

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Topic {
        Unspecified = 0,
        ManageNeuron = 1,
        ExchangeRate = 2,
        NetworkEconomics = 3,
        Governance = 4,
        NodeAdmin = 5,
        ParticipantManagement = 6,
        SubnetManagement = 7,
        NetworkCanisterManagement = 8,
        Kyc = 9,
        NodeProviderRewards = 10,
        SnsDecentralizationSale = 11,
        SubnetRental = 12,
        ReplicaVersionManagement = 13,
        SnsAndCommunityFund = 14,
        ApiBoundaryNodeManagement = 15,
        SubnetReplicaVersionManagement = 16,
        ReplicaVersionManagement2 = 17,
        IcOsVersionElection = 18,
        IcOsVersionDeployment = 19,
        ServiceNervousSystemManagement = 20,
    }

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum ProposalStatus {
        Unspecified = 0,
        Open = 1,
        Rejected = 2,
        Adopted = 3,
        Executed = 4,
        Failed = 5,
    }

    /// NNS Governance ListProposals request (based on IC patterns)
    #[derive(CandidType, Deserialize, Clone, Debug, Default)]
    pub struct ListProposals {
        /// Maximum number of proposals to return. Defaults to 10 if not specified.
        pub limit: Option<u32>,
        /// Return proposals strictly before this proposal ID (for pagination)
        pub before_proposal: Option<u64>,
        /// Exclude proposals with these topics
        pub exclude_topic: Option<Vec<Topic>>,
        /// Include only proposals with these statuses
        pub include_status: Option<Vec<ProposalStatus>>,
        /// Include only proposals rewarding votes on these topics
        pub include_reward_status: Option<Vec<bool>>,
        /// Include only proposals with these topics
        pub include_all_manage_neuron_proposals: Option<bool>,
        /// Omit large fields from the response for performance
        pub omit_large_fields: Option<bool>,
    }

    /// NNS Governance ListProposals response (based on IC patterns)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct ListProposalsResponse {
        /// List of proposals matching the request criteria
        pub proposals: Vec<ProposalInfo>,
        /// Total number of proposals in the governance canister
        pub total_proposal_count: Option<u64>,
    }

    /// Detailed proposal information (simplified version)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct ProposalInfo {
        /// Unique identifier for the proposal
        pub id: u64,
        /// The neuron ID that submitted the proposal
        pub proposer: u64,
        /// Timestamp when the proposal was submitted (nanoseconds since epoch)
        pub proposal_timestamp_seconds: u64,
        /// Current status of the proposal
        pub status: ProposalStatus,
        /// Topic category of the proposal
        pub topic: Topic,
        /// Title/summary of the proposal
        pub title: String,
        /// Detailed description of the proposal
        pub summary: String,
        /// URL to additional proposal documentation
        pub url: String,
    }

    /// GetProposal request for getting individual proposal details
    #[derive(CandidType, Deserialize, Clone, Debug)]
    pub struct GetProposal {
        pub proposal_id: u64,
    }

    /// GetProposal response containing the proposal or error
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct GetProposalResponse {
        pub proposal_info: Option<ProposalInfo>,
    }
}
