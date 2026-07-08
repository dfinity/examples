mod brc20;
mod common;
mod ecdsa;
mod ordinals;
mod p2pkh;
mod p2tr;
mod p2wpkh;
mod runes;
mod schnorr;
mod service;

use ic_cdk::{init, post_upgrade};
use ic_cdk_bitcoin_canister::{
    BlockchainInfo, GetBlockHeadersResponse, GetUtxosResponse, MillisatoshiPerByte, Network,
};
use std::cell::Cell;

/// Runtime configuration shared across all Bitcoin-related operations.
///
/// This struct carries network-specific context:
/// - `network`: The ICP Bitcoin API network enum.
/// - `bitcoin_network`: The corresponding network enum from the `bitcoin` crate, used
///   for address formatting and transaction construction.
/// - `key_name`: The global ECDSA key name used when requesting derived keys or making
///   signatures. Different key names are used locally and when deployed on the IC.
///
/// Note: Both `network` and `bitcoin_network` are needed because ICP and the
/// Bitcoin library use distinct network enum types.
#[derive(Clone, Copy)]
pub struct BitcoinContext {
    pub network: Network,
    pub bitcoin_network: bitcoin::Network,
    pub key_name: &'static str,
}

// Global, thread-local instance of the Bitcoin context.
// Initialized at canister init/upgrade time and reused across all API calls.
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
        Network::Testnet => "test_key_1",
        Network::Mainnet => "key_1",
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

/// Input structure for transferring Rune tokens to another address.
#[derive(candid::CandidType, candid::Deserialize)]
pub struct TransferRuneRequest {
    /// Block height of the etching transaction (first component of the rune ID).
    pub rune_id_block: u64,
    /// Transaction index within that block (second component of the rune ID).
    pub rune_id_tx: u32,
    /// Number of rune tokens to send.
    pub amount: u64,
    /// Bitcoin address of the recipient.
    pub destination_address: String,
}

/// Input structure for transferring BRC-20 tokens to another address.
#[derive(candid::CandidType, candid::Deserialize)]
pub struct TransferBrc20Request {
    /// 4-character BRC-20 ticker symbol (e.g. "DEMO").
    pub tick: String,
    /// Number of tokens to transfer.
    pub amount: u64,
    /// Bitcoin address of the recipient.
    pub destination_address: String,
}

ic_cdk::export_candid!();
