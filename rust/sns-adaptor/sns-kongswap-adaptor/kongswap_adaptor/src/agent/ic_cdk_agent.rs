use crate::agent::EXTERNAL_CALL_TIMEOUT_SECONDS;

use super::{AbstractAgent, Request};
use candid::Principal;
use ic_cdk::call::{CallFailed, CandidDecodeFailed};
use thiserror::Error;

pub struct CdkAgent {}

impl CdkAgent {
    pub fn new() -> Self {
        CdkAgent {}
    }
}

#[derive(Error, Debug)]
pub enum CdkAgentError {
    #[error(transparent)]
    CallFailed(#[from] CallFailed),
    #[error("canister request could not be encoded: {0}")]
    CandidEncode(candid::Error),
    #[error(transparent)]
    CandidDecode(#[from] CandidDecodeFailed),
}

impl AbstractAgent for CdkAgent {
    type Error = CdkAgentError;

    async fn call<R: Request>(
        &mut self,
        canister_id: impl Into<Principal> + Send,
        request: R,
    ) -> Result<R::Response, Self::Error> {
        let raw_args = request.payload().map_err(CdkAgentError::CandidEncode)?;

        let call_response = ic_cdk::call::Call::bounded_wait(canister_id.into(), request.method())
            .take_raw_args(raw_args)
            .change_timeout(EXTERNAL_CALL_TIMEOUT_SECONDS)
            .await?;

        let result = call_response.candid::<<R as Request>::Response>()?;

        Ok(result)
    }
}
