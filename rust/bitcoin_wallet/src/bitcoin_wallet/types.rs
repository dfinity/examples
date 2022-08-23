//! Types used to support the candid API.

use bitcoin::{hashes, util, Address, Transaction};
use ic_btc_types::{OutPoint, Satoshi, Utxo};
use ic_cdk::{
    api::call::RejectionCode,
    export::{
        candid::{CandidType, Deserialize},
        serde::Serialize,
        Principal,
    },
};
use std::collections::{BTreeMap, HashSet};
use std::iter::FromIterator;

pub type Millisatoshi = u64;

#[derive(CandidType, Debug, Deserialize, PartialEq, Clone, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

/// Contains the result of a `get_utxos` call.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct GetUtxosResponse {
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
}

/// ECDSA public key and chain code.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct EcdsaPubKey {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
}

/// Address types supported by the `ic-btc-library`.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum AddressType {
    P2pkh,
    P2sh,
    P2wpkh,
}

/// Errors when processing an `get_p2*_adddress` request.
#[derive(Debug, PartialEq)]
pub enum BitcoinAddressError {
    Hashes(hashes::error::Error),
    UtilKey(util::key::Error),
    UtilAddress(util::address::Error),
}

impl From<hashes::error::Error> for BitcoinAddressError {
    fn from(bitcoin_hashes_error: hashes::error::Error) -> Self {
        BitcoinAddressError::Hashes(bitcoin_hashes_error)
    }
}

impl From<util::key::Error> for BitcoinAddressError {
    fn from(bitcoin_util_key_error: util::key::Error) -> Self {
        BitcoinAddressError::UtilKey(bitcoin_util_key_error)
    }
}

impl From<util::address::Error> for BitcoinAddressError {
    fn from(bitcoin_util_address_error: util::address::Error) -> Self {
        BitcoinAddressError::UtilAddress(bitcoin_util_address_error)
    }
}

/// Error when processing an `add_address` request.
#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct DerivationPathTooLong;

/// Contains the information which UTXOs were added and removed since a given moment.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct UtxosUpdate {
    pub added_utxos: Vec<Utxo>,
    pub removed_utxos: Vec<Utxo>,
}

impl UtxosUpdate {
    pub fn new() -> Self {
        Self {
            added_utxos: vec![],
            removed_utxos: vec![],
        }
    }
}

impl Default for UtxosUpdate {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a `HashSet<Utxo>` from the given UTXOs vector reference.
fn to_hashset(state: &[Utxo]) -> HashSet<Utxo> {
    HashSet::from_iter(state.iter().cloned())
}

/// Returns `state_0`'s UTXOs that aren't in `state_1`.
fn state_difference(state_0: &HashSet<Utxo>, state_1: &HashSet<Utxo>) -> Vec<Utxo> {
    state_0
        .difference(state_1)
        .collect::<Vec<&Utxo>>()
        .into_iter()
        .cloned()
        .collect()
}

impl UtxosUpdate {
    /// Returns an `UtxosUpdate` defined by the changes in the UTXOs set between `seen_state` and `unseen_state`.
    pub fn from_state(seen_state: &[Utxo], unseen_state: &[Utxo]) -> Self {
        let seen_state_hashset = &to_hashset(seen_state);
        let unseen_state_hashset = &to_hashset(unseen_state);
        UtxosUpdate {
            added_utxos: state_difference(unseen_state_hashset, seen_state_hashset),
            removed_utxos: state_difference(seen_state_hashset, unseen_state_hashset),
        }
    }
}

/// Arguments used to call get_utxos_from_args in the agent.
pub struct UtxosArgs {
    pub network: bitcoin::Network,
    pub address: bitcoin::Address,
    pub min_confirmations: u32,
    pub utxos_state: UtxosState,
}

/// Latest utxos retrieved at a given address.
pub struct UtxosResult {
    pub address: bitcoin::Address,
    pub utxos: Vec<Utxo>,
    pub tip_height: u32,
}

/// Represents the last seen state and the unseen state UTXOs for a given `min_confirmations`.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct UtxosState {
    pub seen_state: Vec<Utxo>,
    pub unseen_state: Vec<Utxo>,
    pub min_confirmations: u32,
    pub spent_state: Vec<OutPoint>,
    pub generated_state: Vec<Utxo>,
}

impl UtxosState {
    pub fn new(min_confirmations: u32) -> Self {
        Self {
            seen_state: vec![],
            unseen_state: vec![],
            min_confirmations,
            spent_state: vec![],
            generated_state: vec![],
        }
    }
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct AddressNotTracked;

/// Represents the last seen state and the unseen state balances for a given `min_confirmations`.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct BalanceUpdate {
    pub added_balance: Satoshi,
    pub removed_balance: Satoshi,
}

impl BalanceUpdate {
    pub fn new() -> Self {
        Self {
            added_balance: 0,
            removed_balance: 0,
        }
    }
}

impl Default for BalanceUpdate {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the total value of a UTXOs set.
pub(crate) fn get_balance_from_utxos(utxos: &[Utxo]) -> Satoshi {
    utxos.iter().map(|utxo| utxo.value).sum()
}

impl From<UtxosUpdate> for BalanceUpdate {
    fn from(utxos_update: UtxosUpdate) -> Self {
        Self {
            added_balance: get_balance_from_utxos(&utxos_update.added_utxos),
            removed_balance: get_balance_from_utxos(&utxos_update.removed_utxos),
        }
    }
}

pub(crate) fn from_types_network_to_bitcoin_network(network: Network) -> bitcoin::Network {
    match network {
        Network::Mainnet => bitcoin::Network::Bitcoin,
        Network::Testnet => bitcoin::Network::Testnet,
        Network::Regtest => bitcoin::Network::Regtest,
    }
}

pub(crate) fn from_bitcoin_network_to_ic_btc_types_network(
    network: bitcoin::Network,
) -> ic_btc_types::Network {
    match network {
        bitcoin::Network::Bitcoin => ic_btc_types::Network::Mainnet,
        bitcoin::Network::Testnet => ic_btc_types::Network::Testnet,
        bitcoin::Network::Regtest => ic_btc_types::Network::Regtest,
        // Other cases can't happen see ManagementCanister::new
        _ => panic!(),
    }
}

pub(crate) fn from_bitcoin_network_to_types_network(network: bitcoin::Network) -> Network {
    match network {
        bitcoin::Network::Bitcoin => Network::Mainnet,
        bitcoin::Network::Testnet => Network::Testnet,
        #[cfg(locally)]
        bitcoin::Network::Regtest => Network::Regtest,
        // Other cases can't happen see ManagementCanister::new
        _ => panic!(),
    }
}

/// Needs to use `(String, Network)` to describe an address otherwise there is an ambiguity
/// between testnet and regtest because of the same address prefix.
pub type AddressUsingPrimitives = (String, Network);

/// Represents the Bitcoin agent state used for canister upgrades.
#[derive(CandidType, Debug, Deserialize, PartialEq, Clone)]
pub struct BitcoinWalletState {
    pub network: Network,
    pub main_address_type: AddressType,
    pub ecdsa_pub_key_addresses: BTreeMap<AddressUsingPrimitives, EcdsaPubKey>,
    pub utxos_state_addresses: BTreeMap<AddressUsingPrimitives, UtxosState>,
    pub min_confirmations: u32,
    pub ecdsa_pub_key: EcdsaPubKey,
}

/// The upper bound on the minimum number of confirmations supported by the Bitcoin integration.
pub const MIN_CONFIRMATIONS_UPPER_BOUND: u32 = 6;

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct MinConfirmationsTooHigh;

/// Error when processing an `add_address_with_parameters` request.
#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub enum AddAddressWithParametersError {
    DerivationPathTooLong,
    MinConfirmationsTooHigh,
}

/// Errors when processing a `get_utxos` request.
#[derive(CandidType, Debug)]
pub enum GetUtxosError {
    MinConfirmationsTooHigh,
    ManagementCanisterReject(RejectionCode, String),
}

/// Error when processing a request to the management canister.
#[derive(CandidType, Debug)]
pub struct ManagementCanisterReject(pub RejectionCode, pub String);

/// Errors when processing a `get_current_fee` request.
#[derive(CandidType, Debug)]
pub enum GetCurrentFeeError {
    InvalidPercentile,
    ManagementCanisterReject(RejectionCode, String),
}

impl From<ManagementCanisterReject> for GetCurrentFeeError {
    fn from(ManagementCanisterReject(rejection_code, message): ManagementCanisterReject) -> Self {
        GetCurrentFeeError::ManagementCanisterReject(rejection_code, message)
    }
}

/// Represents the fee request as a percentile in millisatoshis/byte over the last 10,000 transactions.
#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub enum FeeRequest {
    Slow,           // 25th percentile
    Standard,       // 50th percentile
    Fast,           // 75th percentile
    Percentile(u8), // custom percentile
}

/// Arguments used to call get_current_fees_from_args in the agent.
pub struct CurrentFeesArgs {
    pub network: bitcoin::Network,
}

/// Arguments used to call get_initialization_parameters_from_args in the agent.
pub struct InitializationParametersArgs {
    pub key_name: String,
    pub ecdsa_public_key: EcdsaPubKey,
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct InvalidPercentile;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SignWithECDSAReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct ECDSAPublicKey {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum Fee {
    Constant(Satoshi),     // constant fee in millisatoshis for the transaction
    PerByte(Millisatoshi), // constant fee ratio in millisatoshis/byte
    Slow,                  // 25th percentile
    Standard,              // 50th percentile
    Fast,                  // 75th percentile
    Percentile(u8),        // custom percentile
}

impl From<Fee> for FeeRequest {
    fn from(fee: Fee) -> Self {
        match fee {
            Fee::Slow => FeeRequest::Slow,
            Fee::Standard => FeeRequest::Standard,
            Fee::Fast => FeeRequest::Fast,
            Fee::Percentile(percentile) => FeeRequest::Percentile(percentile),
            // Other cases can't happen see multi_transfer
            _ => panic!(),
        }
    }
}

pub type TransactionID = String;

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct TransactionInfo {
    pub id: TransactionID,
    pub utxos_addresses: BTreeMap<AddressUsingPrimitives, Vec<Utxo>>,
    pub fee: Satoshi,
    pub size: u32,
    pub timestamp: u64,
}

#[derive(CandidType, Debug, Deserialize, PartialEq)]
pub struct MultiTransferResult {
    pub transaction_info: TransactionInfo,
    pub generated_utxos_addresses: BTreeMap<AddressUsingPrimitives, Vec<Utxo>>,
    pub height: u32,
}

/// Arguments used to call multi_transfer_from_args in the agent.
#[derive(Debug)]
pub struct MultiTransferArgs {
    pub key_name: String,
    pub ecdsa_pub_key_addresses: BTreeMap<Address, EcdsaPubKey>,
    pub utxos_state_addresses: BTreeMap<Address, UtxosState>,
    pub payouts: BTreeMap<Address, Satoshi>,
    pub change_address: Address,
    pub fee: Fee,
    pub min_confirmations: u32,
    pub replaceable: bool,
    pub network: Network,
}

/// Errors when processing a `multi_transfer` request.
#[derive(CandidType, Debug)]
pub enum MultiTransferError {
    FeeTooLow,
    InvalidPercentile,
    InsufficientBalance,
    MinConfirmationsTooHigh,
    ManagementCanisterReject(RejectionCode, String),
}

impl From<GetCurrentFeeError> for MultiTransferError {
    fn from(get_current_fee_error: GetCurrentFeeError) -> Self {
        match get_current_fee_error {
            GetCurrentFeeError::InvalidPercentile => MultiTransferError::InvalidPercentile,
            GetCurrentFeeError::ManagementCanisterReject(rejection_code, message) => {
                MultiTransferError::ManagementCanisterReject(rejection_code, message)
            }
        }
    }
}

impl From<ManagementCanisterReject> for MultiTransferError {
    fn from(ManagementCanisterReject(rejection_code, error): ManagementCanisterReject) -> Self {
        MultiTransferError::ManagementCanisterReject(rejection_code, error)
    }
}

#[derive(Debug)]
pub struct BuiltTransaction {
    pub transaction: Transaction,
    pub mock_signed_transaction_size: u64,
    pub spending_utxos_addresses: BTreeMap<Address, Vec<Utxo>>,
    pub spending_ecdsa_pub_keys: Vec<EcdsaPubKey>,
    pub fee: Satoshi,
}
