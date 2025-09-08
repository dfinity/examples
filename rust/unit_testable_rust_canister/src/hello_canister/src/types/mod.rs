use candid::{CandidType, Deserialize};
use serde::Serialize;

// =============================================================================
// REQUEST TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IncrementCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DecrementCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ListProposalsRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalInfoRequest {
    pub proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalCountRequest {}

// =============================================================================
// RESPONSE TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetCountResponse {
    pub count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IncrementCountResponse {
    pub new_count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DecrementCountResponse {
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

pub mod nns_governance;
