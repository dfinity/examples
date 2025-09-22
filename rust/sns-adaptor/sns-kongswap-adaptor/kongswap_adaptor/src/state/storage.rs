use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use kongswap_adaptor::treasury_manager::{
    Error, Transaction, TransactionWitness, TreasuryManagerOperation,
};
use serde::Deserialize;
use std::borrow::Cow;

use crate::balances::ValidatedBalances;

/// Configuration state of the KongSwapAdaptor canister (exclusing the `audit_trail`).
#[derive(CandidType, Default, Deserialize)]
pub(crate) enum ConfigState {
    /// This state is only used between wasm module initialization and canister_init().
    #[default]
    Uninitialized,

    /// Includes only `balances` from `KongSwapAdaptor`, since `audit_trail` is stored separately.
    Initialized(ValidatedBalances),
}

impl Storable for ConfigState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("Cannot encode ConfigState"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("Cannot decode ConfigState")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: true,
    };
}

#[derive(CandidType, candid::Deserialize, Clone, Debug)]
pub(crate) struct StableTransaction {
    pub timestamp_ns: u64,
    pub canister_id: Principal,
    pub result: Result<TransactionWitness, Error>,
    pub human_readable: String,
    pub operation: TreasuryManagerOperation,
}

impl Storable for StableTransaction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("Cannot encode StableTransaction"))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("Cannot decode StableTransaction")
    }

    const BOUND: Bound = Bound::Bounded {
        // TODO: Enforce this bound.
        max_size: 2048, // Increased size to accommodate all fields
        is_fixed_size: false,
    };
}

impl From<StableTransaction> for Transaction {
    fn from(item: StableTransaction) -> Self {
        Self {
            timestamp_ns: item.timestamp_ns,
            canister_id: item.canister_id,
            result: item.result,
            purpose: item.human_readable,
            treasury_manager_operation: item.operation,
        }
    }
}

impl From<Transaction> for StableTransaction {
    fn from(item: Transaction) -> Self {
        Self {
            timestamp_ns: item.timestamp_ns,
            canister_id: item.canister_id,
            result: item.result,
            human_readable: item.purpose,
            operation: item.treasury_manager_operation,
        }
    }
}
