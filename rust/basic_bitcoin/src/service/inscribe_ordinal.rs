// This module implements Bitcoin Ordinals inscription functionality.
// Ordinals allow arbitrary data to be inscribed on individual satoshis,
// creating unique digital artifacts on the Bitcoin blockchain.

use crate::{
    common::{get_fee_per_byte, DerivationPath},
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    absolute::LockTime,
    consensus::serialize,
    hashes::Hash,
    opcodes::{all::*, OP_FALSE},
    script::{Builder, PushBytesBuf},
    secp256k1::{PublicKey, Secp256k1},
    sighash::{Prevouts, SighashCache},
    taproot::{ControlBlock, LeafVersion, TaprootBuilder},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction, TxIn,
    TxOut, Txid, Witness, XOnlyPublicKey,
};
use hex::encode;
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, MillisatoshiPerByte,
        SendTransactionRequest,
    },
    trap, update,
};

// Placeholder for the 64-byte Schnorr signature during fee calculation.
// We need this because Bitcoin transaction fees depend on transaction size,
// but we can't sign until we know the fee. This creates a chicken-and-egg problem
// that we solve by using a placeholder of the correct size.
const SIG64_PLACEHOLDER: [u8; 64] = [0u8; 64];

// The amount of satoshis to lock in the inscription output.
// This value must be high enough to ensure the UTXO isn't considered "dust"
// by Bitcoin nodes (typically around 546 sats minimum).
// We use 15,000 sats to be safely above dust limits and account for fees.
const INSCRIPTION_VALUE_SAT: u64 = 15_000;

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
    let mut text_bytes = PushBytesBuf::new();
    text_bytes.extend_from_slice(text.as_bytes()).unwrap();

    // Build the inscription reveal script according to the Ordinals protocol.
    // This script has two execution paths:
    // 1. Normal path: Verify signature against internal_key (for spending)
    // 2. Inscription path: Never executes (inside OP_FALSE OP_IF), but stores our data
    //
    // The inscription envelope (OP_FALSE OP_IF ... OP_ENDIF) ensures the inscription
    // data is included in the witness but never actually executed, preventing errors
    // while still making the data permanently part of the blockchain.
    let reveal_script = Builder::new()
        .push_slice(internal_key.serialize()) // 32-byte x-only key
        .push_opcode(OP_CHECKSIG) // Verify signature for spending
        .push_opcode(OP_FALSE) // Push false to skip inscription data
        .push_opcode(OP_IF) // Begin inscription envelope
        .push_slice(b"ord") // Ordinals protocol marker
        .push_int(1) // Content type field number
        .push_slice(b"text/plain") // MIME type of the inscription
        .push_int(0) // Data push field number
        .push_slice(&text_bytes) // The actual inscription content
        .push_opcode(OP_ENDIF) // End inscription envelope
        .into_script();

    ic_cdk::println!("reveal_script: {}", reveal_script.to_asm_string());

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

    ic_cdk::println!("Reveal script: {:?}", reveal_script.to_bytes());

    // Create the inscription address from our Taproot commitment.
    // This address secretly commits to our inscription script - no one can tell
    // it contains an inscription just by looking at the address.
    let inscription_address =
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
    // This transaction sends funds to the inscription address, "committing" to
    // the inscription without revealing it yet. The inscription data remains
    // hidden until we spend these funds in the reveal transaction.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single, // Use a single UTXO for simplicity
        &inscription_address,
        INSCRIPTION_VALUE_SAT,
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
    // Once confirmed, our funds will be locked at the inscription address.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    // Calculate transaction IDs for tracking and building the reveal transaction.
    // txid: Traditional transaction ID (without witness data)
    // wtxid: Witness transaction ID (includes witness data)
    let commit_txid = signed_transaction.compute_txid();
    let commit_wtxid = signed_transaction.compute_wtxid();

    ic_cdk::println!("commit txid: {}", commit_txid);
    ic_cdk::println!("commit wtxid: {}", commit_wtxid);
    ic_cdk::println!("commit hex: {}", encode(serialize(&signed_transaction)));

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
    let mut reveal_tx = build_reveal_transaction(
        &funding_address, // Where to send remaining funds after inscription
        &reveal_script,
        &control_block,
        &commit_txid,
        fee_per_byte,
    )
    .await;

    // --- Calculate the signature hash for Taproot script-path spending ---
    // Script-path spending requires a different signature than key-path spending.
    // We need to sign a hash that commits to the specific script being executed.
    let mut sighasher = SighashCache::new(&mut reveal_tx);
    let prevout = signed_transaction.output[0].clone(); // The output we're spending from commit tx
    let leaf_hash = TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript);

    // Compute the signature hash that we need to sign.
    // This hash commits to:
    // - The transaction being signed (amounts, outputs, etc.)
    // - The specific script being executed
    // - The input being spent
    // This prevents signature reuse attacks and ensures the signature is only valid
    // for this exact transaction and script.
    let signing_data = sighasher
        .taproot_script_spend_signature_hash(
            0,                          // Input index we're signing
            &Prevouts::All(&[prevout]), // Previous outputs being spent
            leaf_hash,                  // Hash of the script being executed
            TapSighashType::Default,    // Sign all inputs and outputs
        )
        .unwrap()
        .as_byte_array()
        .to_vec();

    ic_cdk::println!("signing_data: {}", hex::encode(signing_data.clone()));

    // Sign the computed hash using our private key.
    // This proves we have the authority to spend these funds according to
    // the rules in our inscription script (which requires a valid signature).
    let raw_signature = sign_with_schnorr(
        ctx.key_name.to_string(),           // Key identifier in the IC canister
        internal_key_path.to_vec_u8_path(), // Same derivation path as commit tx
        None,                               // No merkle root for script-path signing
        signing_data,
    )
    .await;

    // Wrap the raw signature with its sighash type.
    // The sighash type tells verifiers what parts of the transaction this
    // signature commits to (in our case, everything).
    let taproot_script_signature = bitcoin::taproot::Signature {
        signature: bitcoin::secp256k1::schnorr::Signature::from_slice(&raw_signature).unwrap(),
        sighash_type: TapSighashType::Default,
    };

    ic_cdk::println!("sig: {}", hex::encode(&raw_signature));

    // Construct the witness stack for script-path spending.
    // The witness contains all data needed to validate the script execution:
    // 1. Signature: Proves we can satisfy the script's OP_CHECKSIG
    // 2. Script: The actual inscription script to execute
    // 3. Control block: Proves this script is committed to by the address
    //
    // When validators process this transaction, they'll execute our script
    // with the signature, revealing our inscription data in the process.
    let witness = sighasher.witness_mut(0).unwrap();
    witness.clear();
    witness.push(taproot_script_signature.to_vec());
    witness.push(reveal_script.to_bytes());
    witness.push(control_block.serialize());

    ic_cdk::println!("Reveal hex: {}", encode(serialize(&reveal_tx)));

    // Broadcast the reveal transaction.
    // Once confirmed, the inscription is permanently recorded on Bitcoin.
    // The inscription data is now associated with the satoshis that were
    // sent to the funding address, creating a unique digital artifact.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&reveal_tx),
    })
    .await
    .unwrap();

    // Return the reveal transaction ID so users can track their inscription
    reveal_tx.compute_txid().to_string()
}

/// Builds the reveal transaction that spends the inscription output.
///
/// This function handles the complexity of calculating proper fees for the reveal
/// transaction. Since fees depend on transaction size, and size depends on witness
/// data (including signatures), we use an iterative approach to find the right fee.
pub(crate) async fn build_reveal_transaction(
    dst_address: &Address,
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_txid: &Txid,
    fee_per_byte: MillisatoshiPerByte,
) -> Transaction {
    // Fee calculation requires knowing transaction size, but size depends on fee
    // (since fee affects output amount). This creates a circular dependency.
    //
    // Solution: Start with fee=0, build transaction with placeholder witness,
    // calculate actual size, update fee, and repeat until fee stabilizes.
    // This converges quickly (usually 1-2 iterations) because each change in fee
    // only slightly affects the output value encoding.
    let mut fee = 0;
    loop {
        let transaction = build_reveal_transaction_with_fee(
            reveal_script,
            control_block,
            commit_txid,
            dst_address,
            fee,
        )
        .unwrap();

        // Calculate fee based on virtual size (vsize accounts for witness discount)
        let tx_vsize = transaction.vsize() as u64;
        if (tx_vsize * fee_per_byte) / 1000 == fee {
            // Fee calculation has stabilized
            return transaction;
        } else {
            // Update fee and try again
            fee = (tx_vsize * fee_per_byte) / 1000;
        }
    }
}

/// Constructs a reveal transaction with a specific fee amount.
///
/// The reveal transaction has a simple structure:
/// - Input: Spends the inscription output from the commit transaction
/// - Output: Sends remaining value (after fee) to the destination address
/// - Witness: Contains the inscription script and proof of authorization
fn build_reveal_transaction_with_fee(
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_txid: &Txid,
    dst_address: &Address,
    fee: u64,
) -> Result<Transaction, String> {
    // Create input that spends the inscription output.
    // The commit transaction created an output at index 0 that we now spend.
    let input = TxIn {
        previous_output: OutPoint {
            txid: commit_txid.to_owned(),
            vout: 0, // Output index 0 from commit transaction
        },
        script_sig: ScriptBuf::new(), // Empty for Taproot (uses witness instead)
        sequence: Sequence::MAX,      // No relative timelock constraints
        witness: Witness::new(),      // Will be populated with actual witness data
    };

    // Create output that sends remaining funds (minus fee) to destination.
    // The inscription is now "bound" to these satoshis according to ordinal theory.
    let output = TxOut {
        value: Amount::from_sat(INSCRIPTION_VALUE_SAT - fee),
        script_pubkey: dst_address.script_pubkey(),
    };

    // Construct the transaction with witness data.
    // Version 2 is standard for Taproot transactions.
    let mut transaction = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO, // No absolute timelock
        input: vec![input],
        output: vec![output.clone()],
    };

    // Add placeholder witness stack for size calculation.
    // The actual signature will replace the placeholder later.
    // Witness stack order matters for script execution:
    // - Stack bottom: control block (proves script validity)
    // - Stack middle: reveal script (the code to execute)
    // - Stack top: signature (satisfies OP_CHECKSIG in script)
    transaction.input[0].witness.push(SIG64_PLACEHOLDER);
    transaction.input[0].witness.push(reveal_script.to_bytes());
    transaction.input[0].witness.push(control_block.serialize());

    Ok(transaction)
}
