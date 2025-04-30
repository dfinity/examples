mod bitcoin_wallet;
mod ecdsa_api;
mod schnorr_api;

use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_balance, bitcoin_get_block_headers, bitcoin_get_current_fee_percentiles,
        bitcoin_get_utxos, GetBalanceRequest, GetBlockHeadersRequest, GetBlockHeadersResponse,
        GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse, MillisatoshiPerByte,
        Network,
    },
    init, post_upgrade, update,
};
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
    static NETWORK: Cell<Network> = const { Cell::new(Network::Testnet) };

    // The ECDSA key name.
    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}

pub fn init_upgrade(network: Network) {
    NETWORK.with(|n| n.set(network));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network {
            // For local development, we use a special test key.
            Network::Regtest => "dfx_test_key",
            // On the IC we're using a test ECDSA key.
            Network::Mainnet | Network::Testnet => "test_key_1",
        }))
    });

    KEY_NAME.with_borrow(|key_name| assert_ne!(key_name, ""));
}

#[init]
pub fn init(network: Network) {
    init_upgrade(network);
}

#[post_upgrade]
fn upgrade(network: Network) {
    init_upgrade(network);
}

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    bitcoin_get_balance(&GetBalanceRequest {
        address,
        network,
        min_confirmations: None,
    })
    .await
    .unwrap()
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_get_utxos(&GetUtxosRequest {
        address,
        network,
        filter: None,
    })
    .await
    .unwrap()
}

/// Returns the block headers in the given height range.
#[update]
pub async fn get_block_headers(
    start_height: u32,
    end_height: Option<u32>,
) -> GetBlockHeadersResponse {
    let network = NETWORK.with(|n| n.get());
    bitcoin_get_block_headers(&GetBlockHeadersRequest {
        start_height,
        end_height,
        network,
    })
    .await
    .unwrap()
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let network = NETWORK.with(|n| n.get());
    bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest { network })
        .await
        .unwrap()
}

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
pub async fn get_p2pkh_address() -> String {
    let derivation_path: Vec<Vec<u8>> = vec![P2PKH_DERIVATION_PATH.as_bytes().to_vec()];
    let key_name = KEY_NAME.with(|kn| kn.borrow().to_string());
    let network = NETWORK.with(|n| n.get());
    bitcoin_wallet::p2pkh::get_address(network, key_name, derivation_path).await
}

/// Sends the given amount of bitcoin from this canister's p2pkh address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2pkh_address(request: SendRequest) -> String {
    let derivation_path = vec![P2PKH_DERIVATION_PATH.as_bytes().to_vec()];
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
    let derivation_path = vec![P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];
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
    let derivation_path = vec![P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];
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
    let derivation_path = vec![P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];
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
    let derivation_path = vec![P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];
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
    let derivation_path = vec![P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];
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
