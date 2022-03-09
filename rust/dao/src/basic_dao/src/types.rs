use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use std::ops::{Add, AddAssign, SubAssign, Mul};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct BasicDaoStableStorage {
    pub accounts: Vec<Account>,
    pub proposals: Vec<Proposal>,
    pub system_params: SystemParams,
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Tokens {
    pub amount_e8s: u64,
}

impl Add for Tokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tokens { amount_e8s: self.amount_e8s + other.amount_e8s }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Self) {
        self.amount_e8s += other.amount_e8s;
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Self) {
        self.amount_e8s -= other.amount_e8s;
    }
}

impl Mul<u64> for Tokens {
    type Output = Tokens;
    fn mul(self, rhs: u64) -> Self {
        Tokens { amount_e8s: self.amount_e8s * rhs }
    }
}

// The state of a Proposal
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum ProposalState {
    // The proposal is open for voting
    Open,

    // Enough "yes" votes have been cast to accept the proposal, and it will soon be executed
    Accepted,

    // Enough "no" votes have been cast to reject the proposal, and it will not be executed
    Rejected,

    // The proposal is currently being executed
    Executing,

    // The proposal has been successfully executed
    Succeeded,

    // A failure occurred while executing the proposal
    Failed(String),
}

/// A proposal is a proposition to execute an arbitrary canister call
///
/// Token holders can vote to either accept the proposal and execute the given
/// canister call, or vote to reject the proposal and not execute the canister
/// call.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub timestamp: u64,
    pub proposer: Principal,
    pub payload: ProposalPayload,
    pub state: ProposalState,
    pub votes_yes: Tokens,
    pub votes_no: Tokens,
    pub voters: Vec<Principal>,
}

/// The data needed to call a given method on a given canister with given args
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ProposalPayload {
    pub canister_id: Principal,
    pub method: String,
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Vote {
    Yes,
    No,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub tokens: Tokens,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: Principal,
    pub amount: Tokens,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct VoteArgs {
    pub proposal_id: u64,
    pub vote: Vote,
}

#[derive(Clone, Default, Debug, CandidType, Deserialize)]
pub struct SystemParams {
    // The fee incurred by transferring tokens
    pub transfer_fee: Tokens,

    // The amount of tokens needed to vote "yes" to accept, or "no" to reject, a proposal
    pub proposal_vote_threshold: Tokens,

    // The amount of tokens that will be temporarily deducted from the account of
    // a user that submits a proposal. If the proposal is Accepted, this deposit is returned,
    // otherwise it is lost. This prevents users from submitting superfluous proposals.
    pub proposal_submission_deposit: Tokens,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UpdateSystemParamsPayload  {
    pub transfer_fee: Option<Tokens>,
    pub proposal_vote_threshold: Option<Tokens>,
    pub proposal_submission_deposit: Option<Tokens>,
}
