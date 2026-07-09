// This module implements BRC-20 token minting functionality.
// A mint inscription claims a portion of the token supply (up to the per-mint limit
// set in the deploy inscription) and assigns it to the address that holds the inscription.

use crate::{
    brc20::commit_and_reveal,
    common::DerivationPath,
    schnorr::get_schnorr_public_key,
    BTC_CONTEXT,
};
use bitcoin::secp256k1::{PublicKey, Secp256k1};
use bitcoin::{Address, XOnlyPublicKey};
use ic_cdk::{trap, update};

/// Mints BRC-20 tokens by inscribing a mint operation on the Bitcoin blockchain.
///
/// A mint inscription claims tokens from the deployed supply by creating an inscription
/// with the BRC-20 mint JSON at the canister's address. BRC-20 indexers recognise this
/// and credit the canister's balance with the specified amount (subject to the deploy's
/// per-mint limit).
///
/// The minted tokens are credited to the address that holds the mint inscription —
/// in this case, the canister's P2TR key-path address. Tokens must be minted before
/// they can be transferred.
#[update]
pub async fn mint_brc20(tick: String, amount: u64) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if tick.is_empty() {
        trap("BRC-20 ticker cannot be empty");
    }

    if tick.len() != 4 {
        trap("BRC-20 ticker must be exactly 4 characters");
    }

    if amount == 0 {
        trap("Amount must be greater than 0");
    }

    let tick = tick.to_uppercase();

    // BRC-20 mint JSON: claims `amount` tokens from the deployed supply.
    let brc20_json = format!(
        r#"{{"p":"brc-20","op":"mint","tick":"{}","amt":"{}"}}"#,
        tick, amount
    );

    // Mint inscription goes to the canister's own P2TR address to credit its balance.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key_bytes =
        get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key =
        XOnlyPublicKey::from(PublicKey::from_slice(&internal_key_bytes).unwrap());
    let secp = Secp256k1::new();
    let own_address = Address::p2tr(&secp, internal_key, None, ctx.bitcoin_network);

    commit_and_reveal(&ctx, &brc20_json, &own_address).await.0
}
