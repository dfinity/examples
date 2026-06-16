use crate::{common::DerivationPath, p2tr, schnorr::get_schnorr_public_key, BTC_CONTEXT};
use bitcoin::Address;
use ic_cdk::update;

/// Returns a Taproot (P2TR) address with a spendable script path.
///
/// This address supports:
/// - Key path spending via a tweaked internal key (standard Taproot path)
/// - Script path spending via a single script leaf: `<script_leaf_key> CHECKSIG`
///
/// The two public keys are derived from distinct derivation paths:
/// - Internal key: p2tr(0, 1) — used for tweaking and key path spending
/// - Script leaf key: p2tr(0, 2) — used in the script tree (as `<key> OP_CHECKSIG`)
#[update]
pub async fn get_p2tr_script_path_enabled_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Derivation path strategy:
    // We assign fixed address indexes for key roles within Taproot:
    // - Index 0: key-path-only Taproot (no script tree committed)
    // - Index 1: internal key for a Taproot output that includes a script tree
    // - Index 2: script leaf key committed to in the Merkle tree
    let internal_key_path = DerivationPath::p2tr(0, 1);
    let script_leaf_key_path = DerivationPath::p2tr(0, 2);

    // Derive the Schnorr public keys used in this Taproot output:
    // - `internal_key` is used as the untweaked base key (for key path spending)
    // - `script_key` is used inside a Taproot leaf script (for script path spending)
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let script_key = get_schnorr_public_key(&ctx, script_leaf_key_path.to_vec_u8_path()).await;

    // Construct the Taproot leaf script: <script_key> OP_CHECKSIG
    // This is a simple script that allows spending via the script_key alone.
    let taproot_spend_info = p2tr::create_taproot_spend_info(&internal_key, &script_key);

    // Construct and return the final Taproot address.
    // The address encodes the tweaked output key and is network-aware (mainnet, testnet, etc.).
    Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network).to_string()
}
