use bitcoin::{key::Secp256k1, Address, PublicKey, XOnlyPublicKey};
use ic_cdk::update;

use crate::{common::DerivationPath, schnorr::get_schnorr_public_key, BTC_CONTEXT};

/// Returns the P2TR address that holds the canister's rune balances.
///
/// This address uses derivation index 1 (p2tr(0,1)), separate from the main funding
/// address at index 0. All rune premine and transfer change outputs go here, so every
/// UTXO at this address is a rune-bearing output. Use this address to check rune
/// balances in the ord explorer: http://127.0.0.1/address/<ADDRESS>
#[update]
pub async fn get_rune_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let rune_key_path = DerivationPath::p2tr(0, 1);
    let rune_key = get_schnorr_public_key(&ctx, rune_key_path.to_vec_u8_path()).await;
    let rune_key = XOnlyPublicKey::from(PublicKey::from_slice(&rune_key).unwrap());
    let secp = Secp256k1::new();
    Address::p2tr(&secp, rune_key, None, ctx.bitcoin_network).to_string()
}
