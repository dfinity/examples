mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod types;
mod util;

use ic_btc_types::Network;
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

/// Returns the P2PKH address of this canister at a specific derivation path.
#[update]
async fn get_p2pkh_address() -> String {
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
