use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    ordinals::{build_reveal_transaction, create_script_path_witness, INSCRIPTION_OUTPUT_VALUE},
    p2tr,
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BitcoinContext,
};
use bitcoin::{
    consensus::serialize,
    opcodes::{all::*, OP_FALSE},
    script::{Builder, PushBytesBuf},
    secp256k1::{PublicKey, Secp256k1},
    taproot::{LeafVersion, TaprootBuilder},
    Address, ScriptBuf, XOnlyPublicKey,
};
use ic_cdk_bitcoin_canister::{
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
};

/// Builds the BRC-20 reveal script that contains the JSON token deployment data.
///
/// The reveal script follows the same Ordinals protocol format as text inscriptions,
/// but uses "application/json" as the content type and structures the data according
/// to the BRC-20 specification for fungible tokens.
///
/// The script has two execution paths:
/// 1. Normal path: Verify signature against internal_key (for spending authorization)
/// 2. BRC-20 path: Never executes (inside OP_FALSE OP_IF), but stores JSON data
///
/// The inscription envelope (OP_FALSE OP_IF ... OP_ENDIF) ensures the BRC-20
/// JSON data is included in the witness but never actually executed, preventing
/// script errors while still making the token data permanently part of the blockchain.
///
/// Script structure:
/// - internal_key (32 bytes) + OP_CHECKSIG: Enables spending with signature
/// - OP_FALSE + OP_IF: Begin unexecuted inscription envelope
/// - "ord" + field markers: Ordinals protocol identification
/// - "application/json": Content type for BRC-20 data
/// - JSON payload: The actual BRC-20 token deployment data
/// - OP_ENDIF: Close inscription envelope
pub fn build_brc20_reveal_script(internal_key: &XOnlyPublicKey, brc20_json: &str) -> ScriptBuf {
    // Convert the BRC-20 JSON string to bytes for embedding in the script.
    // Bitcoin scripts work with raw bytes, not strings, so we need this conversion.
    let mut inscription_payload = PushBytesBuf::new();
    inscription_payload
        .extend_from_slice(brc20_json.as_bytes())
        .unwrap();

    Builder::new()
        .push_slice(internal_key.serialize()) // 32-byte x-only public key
        .push_opcode(OP_CHECKSIG) // Verify signature for spending authorization
        .push_opcode(OP_FALSE) // Push false to ensure inscription data is never executed
        .push_opcode(OP_IF) // Begin inscription envelope (unreachable code)
        .push_slice(b"ord") // Ordinals protocol marker - identifies this as an inscription
        .push_int(1) // Content type field number (standardized in Ordinals protocol)
        .push_slice(b"text/plain;charset=utf-8") // BRC-20 spec requires text/plain, not application/json
        .push_int(0) // Data field number (standardized in Ordinals protocol)
        .push_slice(&inscription_payload) // The actual BRC-20 JSON token data
        .push_opcode(OP_ENDIF) // End inscription envelope
        .into_script()
}

/// Builds and broadcasts a BRC-20 commit + reveal transaction pair.
///
/// This handles the two-transaction pattern shared by all BRC-20 operations
/// (deploy, mint, transfer-inscription):
/// 1. Commit: Send funds to a Taproot address that commits to the BRC-20 script
/// 2. Reveal: Spend from that address, revealing the BRC-20 JSON in the witness
///
/// The reveal output (the inscription UTXO) is sent to `destination_address`.
/// Returns the reveal transaction ID.
pub(crate) async fn commit_and_reveal(
    ctx: &BitcoinContext,
    brc20_json: &str,
    destination_address: &Address,
) -> String {
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    let reveal_script = build_brc20_reveal_script(&internal_key, brc20_json);

    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .unwrap()
        .finalize(&secp256k1_engine, internal_key)
        .unwrap();

    let commit_address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);
    let funding_address =
        Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network);

    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: funding_address.to_string(),
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    let fee_per_byte = get_fee_per_byte(ctx).await;

    let (commit_tx, prevouts) = p2tr::build_transaction(
        ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single,
        &PrimaryOutput::Address(commit_address, INSCRIPTION_OUTPUT_VALUE),
        fee_per_byte,
    )
    .await;

    let signed_commit = p2tr::sign_transaction_key_spend(
        ctx,
        &funding_address,
        commit_tx,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&signed_commit),
    })
    .await
    .unwrap();

    let control_block = taproot_spend_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();

    let mut reveal_tx = build_reveal_transaction(
        destination_address,
        &reveal_script,
        &control_block,
        &signed_commit.compute_txid(),
        fee_per_byte,
    )
    .await;

    create_script_path_witness(
        ctx,
        &mut reveal_tx,
        &signed_commit.output[0],
        &reveal_script,
        &control_block,
        internal_key_path.to_vec_u8_path(),
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&reveal_tx),
    })
    .await
    .unwrap();

    reveal_tx.compute_txid().to_string()
}
