// This module implements Bitcoin Ordinals inscription functionality.
// Ordinals allow arbitrary data to be inscribed on individual satoshis,
// creating unique digital artifacts on the Bitcoin blockchain.

use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    ordinals::{
        build_ordinal_reveal_script, build_reveal_transaction, create_script_path_witness,
        INSCRIPTION_OUTPUT_VALUE,
    },
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    consensus::serialize,
    script::PushBytesBuf,
    secp256k1::{PublicKey, Secp256k1},
    taproot::{LeafVersion, TaprootBuilder},
    Address, XOnlyPublicKey,
};
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    },
    trap, update,
};

/// Creates an Ordinal inscription on the Bitcoin blockchain.
///
/// Ordinals work by embedding data in a witness script that's revealed when spent.
/// This requires a two-transaction process:
/// 1. Commit transaction: Sends sats to a Taproot address that commits to the inscription script
/// 2. Reveal transaction: Spends those sats, revealing the inscription data in the witness
///
/// The inscription becomes permanently associated with those specific satoshis,
/// which can then be tracked and traded as unique digital artifacts.
#[update]
pub async fn inscribe_ordinal(text: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if text.is_empty() {
        trap("Inscription text cannot be empty");
    }

    // Derive the internal key for our Taproot address.
    // In Taproot, every address has an "internal key" that can be used for key-path spending
    // (direct signature) or combined with scripts for script-path spending.
    // We'll use the same key for both the commit and reveal transactions.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Convert the inscription text to bytes for embedding in the script.
    // Bitcoin scripts work with raw bytes, not strings, so we need this conversion.
    let mut inscription_payload = PushBytesBuf::new();
    inscription_payload
        .extend_from_slice(text.as_bytes())
        .unwrap();

    // Build the inscription reveal script according to the Ordinals protocol.
    // This script has two execution paths:
    // 1. Normal path: Verify signature against internal_key (for spending)
    // 2. Inscription path: Never executes (inside OP_FALSE OP_IF), but stores our data
    //
    // The inscription envelope (OP_FALSE OP_IF ... OP_ENDIF) ensures the inscription
    // data is included in the witness but never actually executed, preventing errors
    // while still making the data permanently part of the blockchain.
    let reveal_script = build_ordinal_reveal_script(&internal_key, &inscription_payload);

    // Create the Taproot commitment that includes our inscription script.
    // Taproot addresses can commit to multiple spending conditions in a Merkle tree.
    // When spending via script path, only the used script needs to be revealed,
    // keeping other scripts private. Here we have just one script (the inscription).
    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone()) // Add inscription script at depth 0
        .unwrap()
        .finalize(&secp256k1_engine, internal_key) // Compute the final tweaked key
        .unwrap();

    // Create the commit address from our Taproot commitment.
    // This address secretly commits to our inscription script - no one can tell
    // it contains an inscription just by looking at the address.
    let commit_address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);

    // Create a simple key-path-only address for funding.
    // We need existing funds to pay for the inscription. This address uses the same
    // internal key but without any script commitments, making it cheaper to spend from.
    let funding_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let funding_key = XOnlyPublicKey::from(PublicKey::from_slice(&funding_key).unwrap());
    let funding_address = Address::p2tr(&secp256k1_engine, funding_key, None, ctx.bitcoin_network);

    // Query for available funds (UTXOs) at our funding address.
    // UTXOs (Unspent Transaction Outputs) are like "coins" in Bitcoin -
    // each represents some amount of bitcoin that hasn't been spent yet.
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: funding_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Build the commit transaction.
    // This transaction sends funds to the commit address, "committing" to
    // the inscription without revealing it yet. The inscription data remains
    // hidden until we spend these funds in the reveal transaction.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single, // An inscription needs to be tied to a single UTXO
        &PrimaryOutput::Address(commit_address, INSCRIPTION_OUTPUT_VALUE),
        fee_per_byte,
    )
    .await;

    // Sign the commit transaction using key-path spending.
    // Since we're spending from a simple Taproot address (no scripts),
    // we can use the more efficient key-path spend with just a signature.
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &funding_address,
        transaction,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![], // No additional script data needed for key-path spend
        sign_with_schnorr,
    )
    .await;

    // Broadcast the commit transaction to the Bitcoin network.
    // Once confirmed, our funds will be locked at the commit address.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    // --- Begin Reveal Transaction ---
    // Now we build the transaction that spends the committed funds and reveals
    // the inscription. This is where the inscription data becomes visible on-chain.

    // Get the control block - this proves our script is part of the Taproot commitment.
    // The control block contains the Merkle proof showing our script's position
    // in the Taproot tree, allowing verifiers to confirm the script is valid.
    let control_block = taproot_spend_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();

    // Build the reveal transaction structure.
    // This transaction spends the output we just created in the commit transaction,
    // revealing the inscription script in the process.
    let mut reveal_transaction = build_reveal_transaction(
        &funding_address, // Where to send remaining funds after inscription
        &reveal_script,
        &control_block,
        &signed_transaction.compute_txid(),
        fee_per_byte,
    )
    .await;

    // Create the script-path witness for the reveal transaction.
    // This involves calculating the signature hash, signing it, and constructing
    // the witness stack with the signature, script, and control block.
    let commit_output = signed_transaction.output[0].clone();
    create_script_path_witness(
        &ctx,
        &mut reveal_transaction,
        &commit_output,
        &reveal_script,
        &control_block,
        internal_key_path.to_vec_u8_path(),
    )
    .await;

    // Broadcast the reveal transaction.
    // Once confirmed, the inscription is permanently recorded on Bitcoin.
    // The inscription data is now associated with the satoshis that were
    // sent to the funding address, creating a unique digital artifact.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&reveal_transaction),
    })
    .await
    .unwrap();

    // Return the reveal transaction ID so users can track their inscription
    reveal_transaction.compute_txid().to_string()
}
