use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use ic_types::PrincipalError;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransferError {
    AccountError(String),
    CallError(String),
    LedgerError(String),
}

impl From<PrincipalError> for TransferError {
    fn from(e: PrincipalError) -> Self {
        TransferError::AccountError(format!("{}", e))
    }
}

impl From<(RejectionCode, String)> for TransferError {
    fn from(e: (RejectionCode, String)) -> Self {
        TransferError::CallError(format!("rejection_code:{:?} error:{}", e.0, e.1))
    }
}

impl From<ic_ledger_types::TransferError> for TransferError {
    fn from(e: ic_ledger_types::TransferError) -> Self {
        TransferError::LedgerError(format!("{}", e))
    }
}
