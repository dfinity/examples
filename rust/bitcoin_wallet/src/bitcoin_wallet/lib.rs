pub mod address_management;
mod bip32_extended_derivation;
mod canister_common;
mod canister_implementation;
mod ecdsa;
mod transaction_management;
mod types;
mod utxo_management;
mod wallet;

use crate::canister_common::ManagementCanister;
use crate::canister_implementation::ManagementCanisterImpl;
use crate::types::{AddressType, Fee, MultiTransferError, Network, TransactionInfo};
use crate::wallet::{
    get_balance_from_args, get_current_fees_from_args, get_initialization_parameters_from_args,
    get_utxos_from_args, multi_transfer_from_args, BitcoinWallet,
};
use bitcoin::Address;
use ic_btc_types::{MillisatoshiPerByte, Satoshi};

use ic_cdk::{
    api::{call::RejectionCode, caller},
    export::{candid::CandidType, Principal},
    trap,
};
use ic_cdk_macros::{query, update};
use std::{cell::RefCell, collections::BTreeMap, str::FromStr};

const MIN_CONFIRMATIONS: u32 = 0;
const NETWORK: Network = Network::Regtest;
const ADDRESS_TYPE: &AddressType = &AddressType::P2pkh;

thread_local! {
    // Each user has its own Bitcoin wallet.
    static WALLETS: RefCell<BTreeMap<Principal, BitcoinWallet<ManagementCanisterImpl>>> = RefCell::new(BTreeMap::default());
    // The default Bitcoin wallet used to set up a wallet instance for each user.
    static DEFAULT_WALLET: RefCell<BitcoinWallet<ManagementCanisterImpl>> =
        RefCell::new(BitcoinWallet::new(
            ManagementCanisterImpl::new(NETWORK),
            ADDRESS_TYPE,
            MIN_CONFIRMATIONS
        ).unwrap());
}

/// Initializes the Bitcoin wallet dapp.
/// This custom endpoint needs to be called once.
/// This endpoint can then be removed in a canister upgrade.
#[update]
async fn initialize() {
    let get_initialization_parameters_args = DEFAULT_WALLET
        .with(|bitcoin_wallet| bitcoin_wallet.borrow().get_initialization_parameters_args());
    let initialization_parameters =
        get_initialization_parameters_from_args(get_initialization_parameters_args)
            .await
            .unwrap();
    DEFAULT_WALLET.with(|bitcoin_wallet| {
        bitcoin_wallet
            .borrow_mut()
            .initialize(initialization_parameters)
    });
}

/// Returns the user's `Principal`.
/// If the user isn't authenticated, then the anonymous principal is returned.
#[query]
fn whoami() -> Principal {
    caller()
}

// All other endpoints than `whoami` trap if the user isn't authenticated.
// TODO (ER-2527) Derive Bitcoin addresses for users (have to derive multiple addresses for a given principal)

/// Returns the user's `Principal` or traps if the user isn't authenticated.
fn get_authenticated_principal() -> Principal {
    let caller_principal = caller();
    if caller_principal == Principal::anonymous() {
        trap("Caller principal wasn't obtained through Internet Identity.")
    }
    caller_principal
}

/// Returns the principal's `Address`.
fn get_principal_address(principal: Principal) -> Address {
    let derivation_path = &[principal.as_slice().to_vec()];
    WALLETS.with(|bitcoin_wallet_users| {
        if bitcoin_wallet_users.borrow().get(&principal).is_none() {
            DEFAULT_WALLET.with(|bitcoin_wallet| {
                bitcoin_wallet_users
                    .borrow_mut()
                    .insert(principal, bitcoin_wallet.borrow().clone());
            })
        }
    });
    WALLETS.with(|bitcoin_wallet_users| {
        bitcoin_wallet_users
            .borrow_mut()
            .get_mut(&principal)
            .unwrap()
            .add_address(derivation_path)
            .unwrap()
    })
}

/// Returns the user's `Address`.
fn get_user_address() -> Address {
    let caller_principal = get_authenticated_principal();
    get_principal_address(caller_principal)
}

/// Returns the user's address as a `String`.
#[update]
async fn get_user_address_str() -> String {
    get_user_address().to_string()
}

/// Returns the user's balance in `Satoshi`s.
#[update]
async fn get_balance() -> Satoshi {
    let principal = get_authenticated_principal();
    let principal_address = &get_principal_address(principal);
    let get_utxos_args = WALLETS.with(|bitcoin_wallet_users| {
        bitcoin_wallet_users.borrow()[&principal]
            .get_utxos_args(principal_address, MIN_CONFIRMATIONS)
    });
    get_balance_from_args(get_utxos_args).await.unwrap()
}

/// Returns the 25th, 50th, and 75th fee percentiles in Millisatoshi/byte over the last
/// 10,000 transactions.
#[update]
async fn get_fees() -> (Satoshi, Satoshi, Satoshi) {
    get_authenticated_principal();
    let get_current_fees_args =
        DEFAULT_WALLET.with(|bitcoin_wallet| bitcoin_wallet.borrow().get_current_fees_args());
    let current_fees = get_current_fees_from_args(get_current_fees_args)
        .await
        .unwrap();
    (current_fees[24], current_fees[49], current_fees[74])
}

/// Sends a transaction, transferring the specified Bitcoin amount to the provided address.
#[update]
async fn transfer(
    address: String,
    amount: Satoshi,
    fee: MillisatoshiPerByte,
    allow_rbf: bool,
) -> Result<TransactionInfo, TransferError> {
    let principal = get_authenticated_principal();
    let principal_address = &get_principal_address(principal);
    let address = Address::from_str(&address)?;
    let payouts = BTreeMap::from([(address, amount)]);

    let get_utxos_args = WALLETS.with(|bitcoin_wallet_users| {
        bitcoin_wallet_users.borrow_mut()[&principal]
            .get_utxos_args(principal_address, MIN_CONFIRMATIONS)
    });
    let get_utxos_result = get_utxos_from_args(get_utxos_args).await.unwrap();
    let multi_transfer_args = WALLETS.with(|bitcoin_wallet_users| {
        let mut bitcoin_wallet_users_mut = bitcoin_wallet_users.borrow_mut();
        let bitcoin_wallet = bitcoin_wallet_users_mut.get_mut(&principal).unwrap();
        bitcoin_wallet.apply_utxos(get_utxos_result);
        bitcoin_wallet
            .get_balance_update(principal_address)
            .unwrap();
        bitcoin_wallet.get_multi_transfer_args(
            &payouts,
            principal_address,
            Fee::PerByte(fee),
            MIN_CONFIRMATIONS,
            allow_rbf,
        )
    });
    let multi_transfer_result = multi_transfer_from_args(multi_transfer_args).await?;
    WALLETS.with(|bitcoin_wallet_users| {
        bitcoin_wallet_users
            .borrow_mut()
            .get_mut(&principal)
            .unwrap()
            .apply_multi_transfer_result(&multi_transfer_result)
    });
    Ok(multi_transfer_result.transaction_info)
}

/// Errors when processing a `transfer` request.
#[derive(CandidType, Debug)]
pub enum TransferError {
    FeeTooLow,
    MalformedDestinationAddress,
    InvalidPercentile,
    InsufficientBalance,
    MinConfirmationsTooHigh,
    ManagementCanisterReject(RejectionCode, String),
}

impl From<MultiTransferError> for TransferError {
    fn from(multi_transfer_error: MultiTransferError) -> Self {
        match multi_transfer_error {
            MultiTransferError::FeeTooLow => TransferError::FeeTooLow,
            MultiTransferError::InvalidPercentile => TransferError::InvalidPercentile,
            MultiTransferError::InsufficientBalance => TransferError::InsufficientBalance,
            MultiTransferError::MinConfirmationsTooHigh => TransferError::MinConfirmationsTooHigh,
            MultiTransferError::ManagementCanisterReject(rejection_code, message) => {
                TransferError::ManagementCanisterReject(rejection_code, message)
            }
        }
    }
}

impl From<bitcoin::util::address::Error> for TransferError {
    fn from(_: bitcoin::util::address::Error) -> Self {
        TransferError::MalformedDestinationAddress
    }
}
