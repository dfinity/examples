use crate::{
    log_err,
    state::{storage::StableTransaction, KongSwapAdaptor},
};
use candid::{CandidType, Principal};
use kongswap_adaptor::agent::{AbstractAgent, Request};
use kongswap_adaptor::requests::CommitStateRequest;
use kongswap_adaptor::treasury_manager::{Error, TransactionWitness, TreasuryManagerOperation};
use std::fmt::Debug;

impl<A: AbstractAgent> KongSwapAdaptor<A> {
    /// Performs the request call and records the transaction in the audit trail.
    pub(crate) async fn emit_transaction<R>(
        &mut self,
        operation: TreasuryManagerOperation,
        canister_id: Principal,
        request: R,
        human_readable: String,
    ) -> Result<R::Ok, Error>
    where
        R: Request + Clone + CandidType + Debug,
    {
        let pending_transaction = StableTransaction {
            timestamp_ns: self.time_ns(),
            result: Ok(TransactionWitness::Pending),
            canister_id,
            human_readable,
            operation,
        };
        let index = self.push_audit_trail_transaction(pending_transaction.clone());

        let call_result = self
            .agent
            .call(canister_id, request.clone())
            .await
            .map_err(|error| {
                Error::new_call(request.method().to_string(), canister_id, error.to_string())
            });

        let (result, function_output) = match call_result {
            Err(err) => (Err(err.clone()), Err(err)),
            Ok(response) => {
                let res = request
                    .transaction_witness(canister_id, response)
                    .map_err(|err| Error::new_backend(err.to_string()));

                match res {
                    Err(err) => (Err(err.clone()), Err(err)),
                    Ok((witness, response)) => (Ok(witness), Ok(response)),
                }
            }
        };

        if let Some(index) = index {
            self.set_audit_trail_transaction_result(
                index,
                StableTransaction {
                    result,
                    ..pending_transaction
                },
            );
        }

        // Self-call to ensure that the state has been committed, to prevent state roll back in case
        // of a panic that occurs before the next (meaningful) async operation. This is recommended:
        // https://internetcomputer.org/docs/building-apps/security/inter-canister-calls#journaling
        if let Err(err) = self.agent.call(self.id, CommitStateRequest {}).await {
            log_err(&format!(
                "Failed to commit state after emitting transaction: {}",
                err
            ));
        }

        function_output
    }
}
