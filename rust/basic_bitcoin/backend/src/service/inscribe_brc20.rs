// This module implements BRC-20 token deployment inscription functionality.
// BRC-20 is a fungible token standard built on top of Bitcoin Ordinals that uses
// structured JSON payloads to represent tokens on the Bitcoin blockchain.

use crate::{
    brc20::commit_and_reveal,
    common::DerivationPath,
    schnorr::get_schnorr_public_key,
    BTC_CONTEXT,
};
use bitcoin::secp256k1::{PublicKey, Secp256k1};
use bitcoin::{Address, XOnlyPublicKey};
use ic_cdk::{trap, update};

/// Creates a BRC-20 token deployment inscription on the Bitcoin blockchain.
///
/// BRC-20 tokens work by embedding structured JSON data in a witness script that's
/// revealed when spent, similar to regular Ordinals but with a standardized token format.
/// This requires the same two-transaction process as Ordinals:
/// 1. Commit transaction: Sends sats to a Taproot address that commits to the BRC-20 script
/// 2. Reveal transaction: Spends those sats, revealing the BRC-20 JSON data in the witness
///
/// For simplicity, this implementation deploys tokens with fixed parameters:
/// - Maximum supply: 21,000,000 tokens
/// - Mint limit: 1,000 tokens per mint operation
///
/// The BRC-20 JSON becomes permanently associated with those specific satoshis,
/// creating the first inscription for that ticker symbol according to BRC-20 rules.
/// In BRC-20, the first deployment of a ticker symbol is considered the canonical one.
#[update]
pub async fn inscribe_brc20(tick: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if tick.is_empty() {
        trap("BRC-20 ticker cannot be empty");
    }

    if tick.len() != 4 {
        trap("BRC-20 ticker must be exactly 4 characters");
    }

    let tick = tick.to_uppercase();

    // BRC-20 deploy JSON: defines the token with max supply and per-mint limit.
    let brc20_json = format!(
        r#"{{"p":"brc-20","op":"deploy","tick":"{}","max":"21000000","lim":"1000"}}"#,
        tick
    );

    // Derive the funding address that receives the inscription output.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key_bytes =
        get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key =
        XOnlyPublicKey::from(PublicKey::from_slice(&internal_key_bytes).unwrap());
    let secp = Secp256k1::new();
    let funding_address = Address::p2tr(&secp, internal_key, None, ctx.bitcoin_network);

    commit_and_reveal(&ctx, &brc20_json, &funding_address).await
}
