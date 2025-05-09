use bitcoin::{key::Secp256k1, Address, PublicKey, XOnlyPublicKey};
use ic_cdk::update;

use crate::{common::DerivationPath, schnorr::get_schnorr_public_key, BTC_CONTEXT};

/// Returns a Taproot (P2TR) address of this smart contract that supports **key path spending only**.
///
/// This address does not commit to a script path (it commits to an unspendable path per BIP-341).
/// It allows spending using a single Schnorr signature corresponding to the internal key.
#[update]
pub async fn get_p2tr_key_path_only_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Derivation path strategy:
    // We assign fixed address indexes for key roles within Taproot:
    // - Index 0: key-path-only Taproot (no script tree committed)
    // - Index 1: internal key for a Taproot output that includes a script tree
    // - Index 2: script leaf key committed to in the Merkle tree
    let internal_key_path = DerivationPath::p2tr(0, 0);

    // Derive the public key used as the internal key (untweaked key path base).
    // This key is used for key path spending only, without any committed script tree.
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;

    // Convert the internal key to an x-only public key, as required by Taproot (BIP-341).
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Create a Taproot address using the internal key only.
    // We pass `None` as the Merkle root, which per BIP-341 means the address commits
    // to an unspendable script path, enabling only key path spending.
    let secp256k1_engine = Secp256k1::new();
    Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network).to_string()
}
