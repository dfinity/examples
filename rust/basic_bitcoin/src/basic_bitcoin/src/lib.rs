mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod types;

use ic_btc_types::{Network, GetUtxosResponse, MillisatoshiPerByte};
use ic_cdk_macros::update;
use std::cell::RefCell;

// The bitcoin network to connect to.
//
// When developing locally this should be `Regtest`.
// When deploying to the IC this should be `Testnet` or `Mainnet`.
const NETWORK: Network = Network::Regtest;

thread_local! {
    // The derivation path to use for ECDSA secp256k1.
    static DERIVATION_PATH: RefCell<Vec<Vec<u8>>>  = RefCell::new(vec![vec![0]]);
}

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    bitcoin_api::get_balance(NETWORK, address).await
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    bitcoin_api::get_utxos(NETWORK, address).await
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    bitcoin_api::get_current_fee_percentiles(NETWORK).await
}

/// Sends a (signed) transaction to the bitcoin network.
#[update]
pub async fn send_transaction(transaction: Vec<u8>) {
    bitcoin_api::send_transaction(NETWORK, transaction).await
}

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
pub async fn get_p2pkh_address() -> String {
    let derivation_path = DERIVATION_PATH.with(|d| d.borrow().clone());
    bitcoin_wallet::get_p2pkh_address(NETWORK, derivation_path).await
}

/// Sends the given amount of bitcoin from this canister to the given address.
#[update]
pub async fn send(request: types::SendRequest) {
    let derivation_path = DERIVATION_PATH.with(|d| d.borrow().clone());
    bitcoin_wallet::send(
        NETWORK,
        derivation_path,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await
}
