mod common;
mod ecdsa;
mod p2pkh;
mod p2tr;
mod p2wpkh;
mod schnorr;
mod service;

use ic_cdk::{bitcoin_canister::Network, init, post_upgrade};
use std::cell::Cell;

/// Runtime configuration shared across all Bitcoin-related operations.
///
/// This struct carries network-specific context:
/// - `network`: The IC Bitcoin API network enum (used with the management canister).
/// - `bitcoin_network`: The corresponding network enum from the `bitcoin` crate, used
///    for address formatting and transaction construction.
/// - `key_name`: The ECDSA key name registered for this canister.
///
/// Note: Both `network` and `bitcoin_network` are needed because the IC and the
/// Bitcoin library use distinct network enum types.
#[derive(Clone, Copy)]
pub struct BitcoinContext {
    pub network: Network,
    pub bitcoin_network: bitcoin::Network,
    pub key_name: &'static str,
}

// Global, thread-local instance of the Bitcoin context.
// This is initialized at canister init/upgrade time and reused across all API calls.
thread_local! {
    static BTC_CONTEXT: Cell<BitcoinContext> = const {
        Cell::new(BitcoinContext {
            network: Network::Testnet,
            bitcoin_network: bitcoin::Network::Testnet,
            key_name: "test_key_1",
        })
    };
}

/// Internal shared init logic used both by init and post-upgrade hooks.
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

/// Canister init hook.
/// Sets up the BitcoinContext based on the given IC Bitcoin network.
#[init]
pub fn init(network: Network) {
    init_upgrade(network);
}

/// Post-upgrade hook.
/// Reinitializes the BitcoinContext with the same logic as `init`.
#[post_upgrade]
fn upgrade(network: Network) {
    init_upgrade(network);
}

/// Input structure for sending Bitcoin.
/// Used across P2PKH, P2WPKH, and P2TR transfer endpoints.
#[derive(candid::CandidType, candid::Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}
