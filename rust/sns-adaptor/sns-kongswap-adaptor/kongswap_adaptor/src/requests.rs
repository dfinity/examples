/// Implementations of the Request trait for various request types used in the KongSwapAdaptor.
use crate::agent::Request;
use crate::treasury_manager::{
    AuditTrail, AuditTrailRequest, BalancesRequest, DepositRequest, TransactionWitness,
    TreasuryManagerResult, WithdrawRequest,
};
use candid::CandidType;

impl Request for DepositRequest {
    fn method(&self) -> &'static str {
        "deposit"
    }

    fn payload(&self) -> Result<Vec<u8>, candid::Error> {
        candid::encode_one(self)
    }

    type Response = TreasuryManagerResult;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: candid::Principal,
        _response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        unimplemented!()
    }
}

impl Request for WithdrawRequest {
    fn method(&self) -> &'static str {
        "withdraw"
    }

    fn payload(&self) -> Result<Vec<u8>, candid::Error> {
        candid::encode_one(self)
    }

    type Response = TreasuryManagerResult;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: candid::Principal,
        _response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        unimplemented!()
    }
}

impl Request for BalancesRequest {
    fn method(&self) -> &'static str {
        "balances"
    }

    fn payload(&self) -> Result<Vec<u8>, candid::Error> {
        candid::encode_one(self)
    }

    type Response = TreasuryManagerResult;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: candid::Principal,
        _response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        unimplemented!()
    }
}

impl Request for AuditTrailRequest {
    fn method(&self) -> &'static str {
        "audit_trail"
    }

    fn payload(&self) -> Result<Vec<u8>, candid::Error> {
        candid::encode_one(self)
    }

    type Response = AuditTrail;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: candid::Principal,
        _response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        unimplemented!()
    }
}

#[derive(CandidType, Debug)]
pub struct CommitStateRequest {}

impl Request for CommitStateRequest {
    fn method(&self) -> &'static str {
        "commit_state"
    }

    fn payload(&self) -> Result<Vec<u8>, candid::Error> {
        Ok(candid::encode_one(&()).unwrap())
    }

    type Response = ();

    type Ok = ();

    fn transaction_witness(
        &self,
        _canister_id: candid::Principal,
        _response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        Err("CommitStateRequest does not have a transaction witness".to_string())
    }
}
