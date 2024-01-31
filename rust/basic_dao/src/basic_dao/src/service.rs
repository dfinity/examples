use crate::types::*;
use crate::env::{Environment, EmptyEnvironment};
use ic_cdk::export::Principal;
use std::collections::HashMap;


/// Implements the Basic DAO interface
pub struct BasicDaoService {
    pub env: Box<dyn Environment>,
    pub accounts: HashMap<Principal, Tokens>,
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
    pub system_params: SystemParams,
}

impl Default for BasicDaoService {
    fn default() -> Self {
        BasicDaoService {
            env: Box::new(EmptyEnvironment {}),
            accounts: HashMap::new(),
            proposals: HashMap::new(),
            next_proposal_id: 0,
            system_params: Default::default(),
        }
    }
}

impl From<BasicDaoStableStorage> for BasicDaoService {
    fn from(stable: BasicDaoStableStorage) -> BasicDaoService {
        let accounts = stable.accounts.clone().into_iter().map(|a| (a.owner, a.tokens)).collect();
        let proposals = stable.proposals.clone().into_iter().map(|p| (p.id, p)).collect();

        BasicDaoService {
            env: Box::new(EmptyEnvironment {}),
            accounts,
            proposals,
            next_proposal_id: 0,
            system_params: stable.system_params,
        }
    }
}

/// Implements the Basic DAO interface
impl BasicDaoService {
    /// Transfer tokens from the caller's account to another account
    pub fn transfer(&mut self, transfer: TransferArgs) -> Result<(), String> {
        let caller = self.env.caller();

        if let Some(account) = self.accounts.get_mut(&caller) {
            if account.clone() < transfer.amount {
                return Err(format!(
                    "Caller's account has insufficient funds to transfer {:?}",
                    transfer.amount
                ));
            } else {
                *account -= transfer.amount + self.system_params.transfer_fee;
                let to_account = self.accounts.entry(transfer.to).or_default();
                *to_account += transfer.amount;
            }
        } else {
            return Err("Caller needs an account to transfer funds".to_string());
        }

        Ok(())
    }

    /// Return the account balance of the caller
    pub fn account_balance(&self) -> Tokens {
        let caller = self.env.caller();
        self.accounts.get(&caller).cloned().unwrap_or_else(|| Default::default())
    }

    /// Lists all accounts
    pub fn list_accounts(&self) -> Vec<Account> {
        self.accounts
            .clone()
            .into_iter()
            .map(|(owner, tokens)| Account { owner, tokens })
            .collect()
    }

    /// Submit a proposal
    ///
    /// A proposal contains a canister ID, method name and method args. If enough users
    /// vote "yes" on the proposal, the given method will be called with the given method
    /// args on the given canister.
    pub fn submit_proposal(&mut self, payload: ProposalPayload) -> Result<u64, String> {
        self.deduct_proposal_submission_deposit()?;

        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = Proposal {
            id: proposal_id,
            timestamp: self.env.now(),
            proposer: self.env.caller(),
            payload,
            state: ProposalState::Open,
            votes_yes: Default::default(),
            votes_no: Default::default(),
            voters: vec![],
        };

        self.proposals.insert(proposal_id, proposal);
        Ok(proposal_id)
    }

    /// Return the proposal with the given ID, if one exists
    pub fn get_proposal(&self, proposal_id: u64) -> Option<Proposal> {
        self.proposals.get(&proposal_id).cloned()
    }

    /// Return the list of all proposals
    pub fn list_proposals(&self) -> Vec<Proposal> {
        self.proposals.values().cloned().collect()
    }

    // Vote on an open proposal
    pub fn vote(&mut self, args: VoteArgs) -> Result<ProposalState, String> {
        let caller = self.env.caller();

        let proposal = self.proposals
            .get_mut(&args.proposal_id)
            .ok_or_else(|| format!("No proposal with ID {} exists", args.proposal_id))?;

        if proposal.state != ProposalState::Open {
            return Err(format!("Proposal {} is not open for voting", args.proposal_id))
        }

        let voting_tokens = self.accounts.get(&caller)
            .ok_or_else(|| "Caller does not have any tokens to vote with".to_string())?
            .clone();

        if proposal.voters.contains(&self.env.caller()) {
            return Err("Already voted".to_string());
        }

        match args.vote {
            Vote::Yes => proposal.votes_yes += voting_tokens,
            Vote::No => proposal.votes_no += voting_tokens,
        }

        proposal.voters.push(caller);

        if proposal.votes_yes >= self.system_params.proposal_vote_threshold {
            // Refund the proposal deposit when the proposal is accepted
            if let Some(account) = self.accounts.get_mut(&proposal.proposer) {
                *account += self.system_params.proposal_submission_deposit.clone();
            }

            proposal.state = ProposalState::Accepted;
        }

        if proposal.votes_no >= self.system_params.proposal_vote_threshold {
            proposal.state = ProposalState::Rejected;
        }

        Ok(proposal.state.clone())
    }

    /// Update system params
    ///
    /// Only callable via proposal execution
    pub fn update_system_params(&mut self, payload: UpdateSystemParamsPayload) {
        if self.env.caller() != self.env.canister_id() {
            return;
        }

        if let Some(transfer_fee) = payload.transfer_fee {
            self.system_params.transfer_fee = transfer_fee;
        }

        if let Some(proposal_vote_threshold) = payload.proposal_vote_threshold {
            self.system_params.proposal_vote_threshold = proposal_vote_threshold;
        }

        if let Some(proposal_submission_deposit) = payload.proposal_submission_deposit {
            self.system_params.proposal_submission_deposit = proposal_submission_deposit;
        }
    }

    /// Update the state of a proposal
    pub fn update_proposal_state(&mut self, proposal_id: u64, new_state: ProposalState) {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            proposal.state = new_state
        }
    }

    /// Deduct the proposal submission deposit from the caller's account
    fn deduct_proposal_submission_deposit(&mut self) -> Result<(), String> {
        let caller = self.env.caller();
        if let Some(account) = self.accounts.get_mut(&caller) {
            if account.clone() < self.system_params.proposal_submission_deposit {
                return Err(format!(
                    "Caller's account must have at least {:?} to submit a proposal",
                    self.system_params.proposal_submission_deposit
                ));
            } else {
                *account -= self.system_params.proposal_submission_deposit.clone();
            }
        } else {
            return Err("Caller needs an account to submit a proposal".to_string());
        }

        Ok(())
    }
}
