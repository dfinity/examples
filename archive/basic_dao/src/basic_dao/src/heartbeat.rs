use ic_cdk_macros::heartbeat;
use crate::SERVICE;
use crate::types::{Proposal, ProposalState};

#[heartbeat]
async fn heartbeat() {
    execute_accepted_proposals().await;
}

/// Execute all accepted proposals
async fn execute_accepted_proposals() {
    let accepted_proposals: Vec<Proposal> = SERVICE.with(|service| {
        service.borrow_mut()
            .proposals
            .values_mut()
            .filter(|proposal| proposal.state == ProposalState::Accepted)
            .map(|proposal| { proposal.state = ProposalState::Executing; proposal.clone() } )
            .collect()
    });

    for proposal in accepted_proposals {
        let state = match execute_proposal(proposal.clone()).await {
            Ok(()) => ProposalState::Succeeded,
            Err(msg) => ProposalState::Failed(msg)
        };

        SERVICE.with(|service| service.borrow_mut().update_proposal_state(proposal.id, state))
    }
}

/// Execute the given proposal
async fn execute_proposal(proposal: Proposal) -> Result<(), String> {
    ic_cdk::api::call::call_raw(
        proposal.payload.canister_id,
        &proposal.payload.method,
        proposal.payload.message.clone(),
        0
    )
        .await
        .map_err(|(code, msg)| {
            format!(
                "Proposal execution failed: \
                canister: {}, method: {}, rejection code: {:?}, message: {}",
                proposal.payload.canister_id,
                &proposal.payload.method,
                code, msg
            )
        })
        .map(|_| ())
}
