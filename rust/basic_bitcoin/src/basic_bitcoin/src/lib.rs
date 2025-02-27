mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod schnorr_api;

use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::{
    bitcoin::{BitcoinNetwork, GetUtxosResponse, MillisatoshiPerByte},
    main::CanisterId,
};
use ic_cdk_macros::{init, update};
use std::cell::{Cell, RefCell};

// Different derivation paths for different addresses to use different keys.
const P2PKH_DERIVATION_PATH: &str = "p2pkh";
const P2TR_DERIVATION_PATH_PREFIX: &str = "p2tr_key_and_script_path";
const P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX: &str = "p2tr_key_path_only";

thread_local! {
    // The bitcoin network to connect to.
    //
    // When developing locally this should be `Regtest`.
    // When deploying to the IC this should be `Testnet` or `Mainnet`.
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);

    // The derivation path to use for the threshold key.
    static DERIVATION_PATH: Vec<Vec<u8>> = vec![];

    // The ECDSA key name.
    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}

#[init]
pub fn init(network: BitcoinNetwork) {
    NETWORK.with(|n| n.set(network));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network {
            // For local development, we use a special test key.
            BitcoinNetwork::Regtest => "dfx_test_key",
            // On the IC we're using a test ECDSA key.
            BitcoinNetwork::Mainnet | BitcoinNetwork::Testnet => "test_key_1",
        }))
    });

    KEY_NAME.with_borrow(|key_name| assert_ne!(key_name, ""));
}

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_balance(network, address).await
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_utxos(network, address).await
}

pub type Height = u32;
pub type BlockHeader = Vec<u8>;

/// A request for getting the block headers for a given height range.
#[derive(CandidType, Debug, Deserialize, PartialEq, Eq)]
pub struct GetBlockHeadersRequest {
    pub start_height: Height,
    pub end_height: Option<Height>,
    pub network: BitcoinNetwork,
}

/// The response returned for a request for getting the block headers from a given height.
#[derive(CandidType, Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct GetBlockHeadersResponse {
    pub tip_height: Height,
    pub block_headers: Vec<BlockHeader>,
}

/// Returns the block headers in the given height range.
#[update]
pub async fn get_block_headers(
    start_height: u32,
    end_height: Option<u32>,
) -> GetBlockHeadersResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_block_headers(network, start_height, end_height).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_current_fee_percentiles(network).await
}

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
pub async fn get_p2pkh_address() -> String {
    let mut derivation_path: Vec<Vec<u8>> = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2PKH_DERIVATION_PATH.as_bytes().to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    bitcoin_wallet::p2pkh::get_address(network, key_name, derivation_path).await
}

/// Sends the given amount of bitcoin from this canister's p2pkh address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2pkh_address(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2PKH_DERIVATION_PATH.as_bytes().to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2pkh::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2tr_address_key_path(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr::send_key_path(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

#[update]
pub async fn send_from_p2tr_address_script_path(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr::send_script_path(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_key_only_address() -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX.as_bytes().to_vec());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());

    bitcoin_wallet::p2tr_key_only::get_address(network, key_name, derivation_path)
        .await
        .to_string()
}

/// Sends the given amount of bitcoin from this canister's p2tr address to the
/// given address. Returns the transaction ID.
///
/// IMPORTANT: This function uses an untweaked key as the spending key.
///
/// WARNING: This function is not suited for multi-party scenarios where
/// multiple keys are used for spending.
#[update]
pub async fn send_from_p2tr_key_only_address(request: SendRequest) -> String {
    let mut derivation_path = DERIVATION_PATH.with(|d| d.clone());
    derivation_path.push(P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX.as_bytes().to_vec());
    let network = NETWORK.with(|n| n.get());
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let tx_id = bitcoin_wallet::p2tr_key_only::send(
        network,
        derivation_path,
        key_name,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}

fn mgmt_canister_id() -> CanisterId {
    // Management canister ID. Can be replaced for cheaper testing.
    candid::Principal::management_canister()
}
