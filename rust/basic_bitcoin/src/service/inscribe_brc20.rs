// This module implements BRC-20 token deployment inscription functionality.
// BRC-20 is a fungible token standard built on top of Bitcoin Ordinals that uses
// structured JSON payloads to represent tokens on the Bitcoin blockchain.

use crate::{
    brc20::build_brc20_reveal_script,
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    ordinals::{build_reveal_transaction, create_script_path_witness, INSCRIPTION_OUTPUT_VALUE},
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    consensus::serialize,
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

    // Derive the internal key for our Taproot address.
    // In Taproot, every address has an "internal key" that can be used for key-path spending
    // (direct signature) or combined with scripts for script-path spending.
    // We'll use the same key for both the commit and reveal transactions.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Convert ticker to uppercase as per BRC-20 convention and create the deployment JSON.
    // BRC-20 uses a specific JSON structure for token operations:
    // - "p": Protocol identifier (always "brc-20")
    // - "op": Operation type ("deploy", "mint", or "transfer")
    // - "tick": 4-character ticker symbol
    // - "max": Maximum supply for deploy operations
    // - "lim": Mint limit per operation for deploy operations
    let tick = tick.to_uppercase();
    let brc20_json = format!(
        r#"{{"p":"brc-20","op":"deploy","tick":"{}","max":"21000000","lim":"1000"}}"#,
        tick
    );

    // Build the BRC-20 reveal script according to the Ordinals protocol.
    // This script has two execution paths:
    // 1. Normal path: Verify signature against internal_key (for spending)
    // 2. BRC-20 path: Never executes (inside OP_FALSE OP_IF), but stores our JSON data
    //
    // The inscription envelope (OP_FALSE OP_IF ... OP_ENDIF) ensures the BRC-20
    // JSON data is included in the witness but never actually executed, preventing errors
    // while still making the token data permanently part of the blockchain.
    let reveal_script = build_brc20_reveal_script(&internal_key, &brc20_json);

    // Create the Taproot commitment that includes our BRC-20 script.
    // Taproot addresses can commit to multiple spending conditions in a Merkle tree.
    // When spending via script path, only the used script needs to be revealed,
    // keeping other scripts private. Here we have just one script (the BRC-20 deployment).
    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone()) // Add BRC-20 script at depth 0
        .unwrap()
        .finalize(&secp256k1_engine, internal_key) // Compute the final tweaked key
        .unwrap();

    // Create the commit address from our Taproot commitment.
    // This address secretly commits to our BRC-20 script - no one can tell
    // it contains a token deployment just by looking at the address.
    let commit_address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);

    // Create a simple key-path-only address for funding.
    // We need existing funds to pay for the BRC-20 deployment. This address uses the same
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
    // the BRC-20 deployment without revealing it yet. The token data remains
    // hidden until we spend these funds in the reveal transaction.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single, // A BRC-20 token needs to be tied to a single UTXO
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
    // the BRC-20 token deployment. This is where the token data becomes visible on-chain.

    // Get the control block - this proves our script is part of the Taproot commitment.
    // The control block contains the Merkle proof showing our script's position
    // in the Taproot tree, allowing verifiers to confirm the script is valid.
    let control_block = taproot_spend_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();

    // Build the reveal transaction structure.
    // This transaction spends the output we just created in the commit transaction,
    // revealing the BRC-20 script in the process.
    let mut reveal_transaction = build_reveal_transaction(
        &funding_address, // Where to send remaining funds after BRC-20 deployment
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
    // Once confirmed, the BRC-20 token is permanently deployed on Bitcoin.
    // The token JSON data is now associated with the satoshis that were
    // sent to the funding address, creating the canonical deployment for this ticker.
    // According to BRC-20 rules, this becomes the authoritative token definition
    // if it's the first deployment inscription for this ticker symbol.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&reveal_transaction),
    })
    .await
    .unwrap();

    // Return the reveal transaction ID so users can track their BRC-20 deployment
    reveal_transaction.compute_txid().to_string()
}
