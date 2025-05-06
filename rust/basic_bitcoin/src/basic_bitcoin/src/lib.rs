mod common;
mod ecdsa;
mod p2pkh;
mod p2tr;
mod p2wpkh;
mod schnorr;
mod service;

use ic_cdk::{bitcoin_canister::Network, init, post_upgrade};
use std::cell::Cell;

#[derive(Clone, Copy)]
pub struct BitcoinContext {
    network: Network,
    bitcoin_network: bitcoin::Network,
    key_name: &'static str,
}

thread_local! {
    static BTC_CONTEXT: Cell<BitcoinContext> = const {
        Cell::new(BitcoinContext {
            network: Network::Testnet,
            bitcoin_network: bitcoin::Network::Testnet,
            key_name: "test_key_1",
        })
    };
}

fn init_upgrade(network: Network) {
    let key_name = match network {
        Network::Regtest => "dfx_test_key",
        Network::Mainnet | Network::Testnet => "test_key_1",
    };

    let bitcoin_network = match network {
        Network::Mainnet => bitcoin::Network::Bitcoin,
        Network::Testnet => bitcoin::Network::Testnet,
        Network::Regtest => bitcoin::Network::Regtest,
    };

    BTC_CONTEXT.with(|ctx| {
        ctx.set(BitcoinContext {
            network,
            bitcoin_network,
            key_name,
        })
    });
}

#[init]
pub fn init(network: Network) {
    init_upgrade(network);
}

#[post_upgrade]
fn upgrade(network: Network) {
    init_upgrade(network);
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}
