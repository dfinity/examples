mod bitcoin_api;

use candid::Nat;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::{init, update};
use num_bigint::BigUint;
use std::{cell::Cell, time::Duration};

thread_local! {
    // The bitcoin network to connect to.
    //
    // When developing locally this should be `Regtest`.
    // When deploying to the IC this should be `Testnet` or `Mainnet`.
    static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);
}

#[init]
pub fn init(network: BitcoinNetwork) {
    NETWORK.set(network);
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> Nat {
    let start_ns = ic_cdk::api::time();
    bitcoin_api::get_utxos(NETWORK.get(), address).await;
    let end_ns = ic_cdk::api::time();
    BigUint::from(Duration::from_nanos(end_ns.saturating_sub(start_ns)).as_millis()).into()
}
