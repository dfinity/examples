// This module implements Bitcoin Runes token etching functionality.
// Runes are fungible tokens on Bitcoin that use OP_RETURN outputs
// with OP_13 markers to store token metadata directly on-chain.

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

/// Creates a new Rune token on the Bitcoin blockchain.
///
/// Runes work by embedding token metadata directly into Bitcoin transactions using
/// OP_RETURN outputs with OP_13 markers. Unlike Ordinals, which require a two-transaction
/// commit/reveal process, Runes are etched in a single transaction - the token is
/// created immediately when the transaction is confirmed.
///
/// For simplicity, this implementation creates a basic rune with fixed parameters:
/// - No divisibility (whole units only, no decimal places)
/// - A premine of 1,000,000 units (all supply minted to the etcher)
/// - No open minting terms (supply is fixed at creation)
/// - Unicode coin symbol (🪙) for display purposes
///
/// The rune metadata becomes permanently recorded in the Bitcoin blockchain
/// as part of the OP_RETURN output, creating a new fungible token.
#[update]
pub async fn etch_rune(name: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Validate rune name according to protocol rules.
    // Runes use strict naming conventions for consistency.
    if name.is_empty() {
        trap("Rune name cannot be empty");
    }

    if name.len() > 28 {
        trap("Rune name cannot exceed 28 characters");
    }

    if !name.chars().all(|c| c.is_ascii_uppercase()) {
        trap("Rune name must contain only uppercase letters A-Z");
    }

    // Derive the internal key for our Taproot address.
    // Since rune data goes in OP_RETURN (not script), we use simple key-path spending.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Create our Taproot address for funding the rune etching.
    // No script commitments needed since rune data goes in OP_RETURN output.
    let secp256k1_engine = Secp256k1::new();
    let own_address = Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network);

    // Query for available funds (UTXOs) to pay for the rune etching.
    // We need existing bitcoin to cover transaction fees and any change.
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Create the rune etching configuration with fixed parameters.
    // This defines all the token properties that will be permanently recorded.
    let etching = Etching {
        divisibility: 0,    // No decimal places (whole units only)
        premine: 1_000_000, // Mint 1M units to the etcher (fixed supply)
        rune_name: name.clone(),
        symbol: Some('🪙'), // Unicode coin symbol for display
        terms: None,        // No open minting allowed
        turbo: false,       // Standard etching mode
        spacers: 0,         // No visual spacers in the name
    };

    // Build the runestone script containing the rune metadata.
    // This creates the OP_RETURN output that defines the new token.
    let runestone_script = build_etching_script(&etching)
        .unwrap_or_else(|e| trap(format!("Failed to build runestone: {}", e)));

    // Build the rune etching transaction.
    // The transaction includes an OP_RETURN output with the encoded runestone.
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

    // Sign the rune etching transaction using key-path spending.
    // Simple signature since we're not using any script commitments.
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

    // Broadcast the transaction to the Bitcoin network.
    // Once confirmed, the rune is permanently etched and the tokens are minted.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    // Return the transaction ID so users can track their rune etching.
    signed_transaction.compute_txid().to_string()
}
