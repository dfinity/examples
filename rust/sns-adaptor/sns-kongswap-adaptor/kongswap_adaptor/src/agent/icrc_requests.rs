//! This module contains implementations of the `Request` trait for some ICRC-1 and ICRC-2
//! functions used in the KongSwapAdaptor canister. See https://github.com/dfinity/ICRC-1

use crate::audit::serialize_reply;

use super::Request;
use crate::treasury_manager::{TransactionWitness, Transfer};
use candid::{CandidType, Error, Nat, Principal};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{TransferArg, TransferError};
use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;

impl Request for Account {
    fn method(&self) -> &'static str {
        "icrc1_balance_of"
    }

    fn payload(&self) -> Result<Vec<u8>, Error> {
        candid::encode_one(self)
    }

    type Response = Nat;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        let human_readable = format!("Balance of account {}: {}", self, response);

        Ok((TransactionWitness::NonLedger(human_readable), response))
    }
}

impl Request for TransferArg {
    fn method(&self) -> &'static str {
        "icrc1_transfer"
    }

    fn payload(&self) -> Result<Vec<u8>, Error> {
        candid::encode_one(self)
    }

    type Response = Result<Nat, TransferError>;

    type Ok = Nat;

    fn transaction_witness(
        &self,
        canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        let block_index = response.map_err(|err| err.to_string())?;

        let ledger_canister_id = canister_id.to_string();
        let amount_decimals = self.amount.clone();

        let witness = TransactionWitness::Ledger(vec![Transfer {
            ledger_canister_id,
            amount_decimals,
            block_index: block_index.clone(),
            sender: None,
            receiver: None,
        }]);

        Ok((witness, block_index))
    }
}

impl Request for ApproveArgs {
    fn method(&self) -> &'static str {
        "icrc2_approve"
    }

    fn payload(&self) -> Result<Vec<u8>, Error> {
        candid::encode_one(self)
    }

    type Response = Result<Nat, ApproveError>;

    type Ok = Nat;

    fn transaction_witness(
        &self,
        canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        let block_index = response.map_err(|err| err.to_string())?;

        let ledger_canister_id = canister_id.to_string();
        let amount_decimals = self.amount.clone();

        let witness = TransactionWitness::Ledger(vec![Transfer {
            ledger_canister_id,
            amount_decimals,
            block_index: block_index.clone(),
            sender: None,
            receiver: None,
        }]);

        Ok((witness, block_index))
    }
}

#[derive(CandidType, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Icrc1MetadataRequest {}

impl Request for Icrc1MetadataRequest {
    fn method(&self) -> &'static str {
        "icrc1_metadata"
    }

    fn payload(&self) -> Result<Vec<u8>, Error> {
        candid::encode_one(())
    }

    type Response = Vec<(String, MetadataValue)>;

    type Ok = Self::Response;

    fn transaction_witness(
        &self,
        _canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        let response_str = serialize_reply(&response);
        Ok((TransactionWitness::NonLedger(response_str), response))
    }
}

impl Request for TransferFromArgs {
    fn method(&self) -> &'static str {
        "icrc2_transfer_from"
    }

    fn payload(&self) -> Result<Vec<u8>, Error> {
        candid::encode_one(self)
    }

    type Response = Result<Nat, TransferFromError>;

    type Ok = Nat;

    fn transaction_witness(
        &self,
        canister_id: Principal,
        response: Self::Response,
    ) -> Result<(TransactionWitness, Self::Ok), String> {
        let block_index = response.map_err(|err| err.to_string())?;

        let ledger_canister_id = canister_id.to_string();
        let amount_decimals = self.amount.clone();

        let witness = TransactionWitness::Ledger(vec![Transfer {
            ledger_canister_id,
            amount_decimals,
            block_index: block_index.clone(),
            sender: None,
            receiver: None,
        }]);

        Ok((witness, block_index))
    }
}
