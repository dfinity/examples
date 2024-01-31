mod env;
mod heartbeat;
mod init;
mod service;
mod types;

use crate::service::BasicDaoService;
use crate::types::*;
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    static SERVICE: RefCell<BasicDaoService> = RefCell::default();
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_system_params() -> SystemParams {
    SERVICE.with(|service| service.borrow().system_params.clone())
}

#[update]
#[ic_cdk::export::candid::candid_method]
fn transfer(args: TransferArgs) -> Result<(), String> {
    SERVICE.with(|service| service.borrow_mut().transfer(args))
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn account_balance() -> Tokens {
    SERVICE.with(|service| service.borrow().account_balance())
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_accounts() -> Vec<Account> {
    SERVICE.with(|service| service.borrow().list_accounts())
}

#[update]
#[ic_cdk::export::candid::candid_method]
fn submit_proposal(proposal: ProposalPayload) -> Result<u64, String> {
    SERVICE.with(|service| service.borrow_mut().submit_proposal(proposal))
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_proposal(proposal_id: u64) -> Option<Proposal> {
    SERVICE.with(|service| service.borrow().get_proposal(proposal_id))
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_proposals() -> Vec<Proposal> {
    SERVICE.with(|service| service.borrow().list_proposals())
}

#[update]
#[ic_cdk::export::candid::candid_method]
fn vote(args: VoteArgs) -> Result<ProposalState, String> {
    SERVICE.with(|service| service.borrow_mut().vote(args))
}

#[update]
#[ic_cdk::export::candid::candid_method]
fn update_system_params(payload: UpdateSystemParamsPayload) {
    SERVICE.with(|service| service.borrow_mut().update_system_params(payload))
}

ic_cdk::export::candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
