use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::{StableCell, StableVec, Storable};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::Memo;
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::approve::ApproveError;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use icrc_ledger_types::icrc3::transactions::Transaction;
use std::borrow::Cow;

type VMem = VirtualMemory<DefaultMemoryImpl>;
pub type Tokens = Nat;

pub struct State {
    pub configuration: ConfigCell,
    pub transaction_log: TransactionLog,
}

// The struct that holds the token ledger's settings
pub type ConfigCell = StableCell<Configuration, VMem>;
#[derive(Debug, Default, CandidType, Deserialize)]
pub struct Configuration {
    pub token_name: String,
    pub token_symbol: String,
    pub token_logo: String,
    pub transfer_fee: Tokens,
    pub decimals: u8,
    pub minting_account: Option<Account>,
    pub token_created: bool,
}

// To persist the configuration we need to store it in stable storage.
// Storable describes how the configuration is serialized while stored in stable storage.
// Here, we use Candid en/de-coding. It is not too space efficient, but it is simple to do.
impl Storable for Configuration {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(
            Encode!(&self)
                .expect("failed to serialize Configuration")
                .into(),
        )
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, Configuration).expect("failed to deserialize Configuration")
    }
    const BOUND: Bound = Bound::Unbounded;
}

// To persist the transactions we need to store them in stable storage.
// Storable describes how the transactions are serialized while stored in stable storage.
// Here, we use Candid en/de-coding. It is not too space efficient, but it is simple to do.
pub type TransactionLog = StableVec<StorableTransaction, VMem>;
pub struct StorableTransaction(pub Transaction);
impl Storable for StorableTransaction {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(
            Encode!(&self.0)
                .expect("failed to serialize Transaction")
                .into(),
        )
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Decode!(&bytes, Transaction).expect("failed to deserialize Transaction"))
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: 1000,
        is_fixed_size: false,
    };
}

#[derive(Debug)]
pub struct TxInfo {
    pub from: Account,
    pub to: Option<Account>,
    pub amount: Tokens,
    pub spender: Option<Account>,
    pub memo: Option<Memo>,
    pub fee: Option<Tokens>,
    pub created_at_time: Option<u64>,
    pub expected_allowance: Option<Tokens>,
    pub expires_at: Option<u64>,
    pub is_approval: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SupportedStandard {
    pub name: String,
    pub url: String,
}

pub fn to_approve_error(err: TransferError) -> ApproveError {
    match err {
        TransferError::BadFee { expected_fee } => ApproveError::BadFee { expected_fee },
        TransferError::TooOld => ApproveError::TooOld,
        TransferError::CreatedInFuture { ledger_time } => {
            ApproveError::CreatedInFuture { ledger_time }
        }
        TransferError::TemporarilyUnavailable => ApproveError::TemporarilyUnavailable,
        TransferError::Duplicate { duplicate_of } => ApproveError::Duplicate { duplicate_of },
        TransferError::GenericError {
            error_code,
            message,
        } => ApproveError::GenericError {
            error_code,
            message,
        },
        TransferError::BadBurn { .. } | TransferError::InsufficientFunds { .. } => {
            ic_cdk::trap("Bug: cannot transform TransferError into ApproveError")
        }
    }
}

pub fn to_transfer_from_error(err: TransferError) -> TransferFromError {
    match err {
        TransferError::BadFee { expected_fee } => TransferFromError::BadFee { expected_fee },
        TransferError::TooOld => TransferFromError::TooOld,
        TransferError::CreatedInFuture { ledger_time } => {
            TransferFromError::CreatedInFuture { ledger_time }
        }
        TransferError::TemporarilyUnavailable => TransferFromError::TemporarilyUnavailable,
        TransferError::Duplicate { duplicate_of } => TransferFromError::Duplicate { duplicate_of },
        TransferError::GenericError {
            error_code,
            message,
        } => TransferFromError::GenericError {
            error_code,
            message,
        },
        TransferError::InsufficientFunds { balance } => {
            TransferFromError::InsufficientFunds { balance }
        }
        TransferError::BadBurn { min_burn_amount } => {
            TransferFromError::BadBurn { min_burn_amount }
        }
    }
}

#[derive(Debug, CandidType, Deserialize)]
pub struct CreateTokenArgs {
    pub token_name: String,
    pub token_symbol: String,
    pub initial_supply: Nat,
    pub token_logo: String,
}
