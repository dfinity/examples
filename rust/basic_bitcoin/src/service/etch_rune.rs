// This module implements Bitcoin Runes etching functionality.
// Runes are fungible tokens on Bitcoin that use OP_RETURN outputs
// with OP_13 markers to store token metadata.

use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    p2tr,
    runes::{build_etching_script, Etching},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    consensus::serialize,
    secp256k1::{PublicKey, Secp256k1},
    Address, XOnlyPublicKey,
};
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    },
    trap, update,
};

/// Creates a new Rune on the Bitcoin blockchain.
///
/// Runes are etched in a single transaction that includes an OP_RETURN output
/// with the encoded runestone. Unlike Ordinals, Runes don't require a two-step
/// commit/reveal process - the etching happens immediately when the transaction
/// is confirmed.
///
/// This implementation creates a simple rune with:
/// - No divisibility (whole units only)
/// - A premine of 1,000,000 units (all minted to the etcher)
/// - No open minting terms (supply is fixed)
#[update]
pub async fn etch_rune(name: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Validate rune name
    if name.is_empty() {
        trap("Rune name cannot be empty");
    }

    if name.len() > 28 {
        trap("Rune name cannot exceed 28 characters");
    }

    if !name.chars().all(|c| c.is_ascii_uppercase()) {
        trap("Rune name must contain only uppercase letters A-Z");
    }

    // Derive the key for our Taproot address.
    // We use key-path spending since rune data goes in OP_RETURN.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Create our Taproot address (no script commitments needed for runes)
    let secp256k1_engine = Secp256k1::new();
    let own_address = Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network);

    // Query for available UTXOs
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Create the etching configuration
    let etching = Etching {
        divisibility: 0,    // No decimal places
        premine: 1_000_000, // Mint all supply to etcher
        rune_name: name.clone(),
        symbol: Some('🪙'), // Unicode coin symbol
        terms: None,        // No open minting
        turbo: false,       // Not using turbo mode
        spacers: 0,         // No spacers in name
    };

    // Build the runestone script
    let runestone_script = build_etching_script(&etching)
        .unwrap_or_else(|e| trap(&format!("Failed to build runestone: {}", e)));

    // Build the transaction
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &own_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single,
        &PrimaryOutput::OpReturn(runestone_script),
        fee_per_byte,
    )
    .await;

    // Sign the transaction.
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &own_address,
        transaction,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
    )
    .await;

    // Send the transaction to the Bitcoin API.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    // Return the transaction ID.
    signed_transaction.compute_txid().to_string()
}
