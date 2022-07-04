mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod types;
mod util;
use bitcoin::util::psbt::serialize::Serialize as _;
use bitcoin::{
    blockdata::script::Builder, hashes::Hash, Address, AddressType, OutPoint, Script, SigHashType,
    Transaction, TxIn, TxOut, Txid,
};
use ic_btc_types::{Network, Utxo};
use ic_cdk::{print, trap};
use ic_cdk_macros::update;
use std::str::FromStr;
use types::*;

const DERIVATION_PATH: &[&[u8]] = &[&[0]];

