use crate::treasury_manager::TransactionWitness;
use candid::{CandidType, Principal};
use serde::de::DeserializeOwned;
use std::{error::Error, fmt::Display, future::Future};

pub mod ic_cdk_agent;
pub mod icrc_requests;
pub mod mock_agent;

use std::fmt::Debug;

pub(crate) const EXTERNAL_CALL_TIMEOUT_SECONDS: u32 = 15 * 60; // A time out of 15 minutes for requests.

/// This trait represents a request that can be sent to a canister.
/// It defines the method name, payload, response type, and how to extract
/// a transaction witness from the response.
pub trait Request: Send {
    fn method(&self) -> &'static str;
    fn payload(&self) -> Result<Vec<u8>, candid::Error>;

    type Response: CandidType + DeserializeOwned + Send;

    /// The type representing the successful response from the canister.
    ///
    /// Either the same, or a sub-structure of `Response`.
    type Ok: CandidType + DeserializeOwned + Send;

    fn transaction_witness(
        &self,
        canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String>;
}

/// An abstract agent that can make calls to canisters.
/// Defining this trait allows dependency injection of different agent implementations,
/// such as a mock agent for testing purposes.
pub trait AbstractAgent: Send + Sync {
    type Error: Display + Send + Error + 'static;

    fn call<R: Request + CandidType + Debug>(
        &mut self,
        canister_id: impl Into<Principal> + Send,
        request: R,
    ) -> impl Future<Output = Result<R::Response, Self::Error>> + Send;
}
