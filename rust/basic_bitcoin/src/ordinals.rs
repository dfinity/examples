use crate::{schnorr::sign_with_schnorr, BitcoinContext};
use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    opcodes::{all::*, OP_FALSE},
    script::{Builder, PushBytesBuf},
    sighash::{Prevouts, SighashCache},
    taproot::{ControlBlock, LeafVersion},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction, TxIn,
    TxOut, Txid, Witness, XOnlyPublicKey,
};
use ic_cdk::bitcoin_canister::MillisatoshiPerByte;

// Placeholder for the 64-byte Schnorr signature during fee estimation.
// We need this because Bitcoin transaction fees depend on transaction size,
// but we can't sign until we know the fee. This creates a chicken-and-egg problem
// that we solve by using a placeholder of the correct size.
const SCHNORR_SIGNATURE_PLACEHOLDER: [u8; 64] = [0u8; 64];

// The amount of satoshis to lock in the inscription output.
// This value must be high enough to ensure the UTXO isn't considered "dust"
// by Bitcoin nodes (typically around 546 sats minimum).
// We use 15,000 sats to be safely above dust limits and account for fees.
pub const INSCRIPTION_OUTPUT_VALUE: u64 = 15_000;

/// Builds the reveal transaction that spends the commit output.
///
/// This function handles the complexity of calculating proper fees for the reveal
/// transaction. Since fees depend on transaction size, and size depends on witness
/// data (including signatures), we use an iterative approach to find the right fee.
pub(crate) async fn build_reveal_transaction(
    destination_address: &Address,
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_tx_id: &Txid,
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
            commit_tx_id,
            destination_address,
            fee,
        )
        .unwrap();

        // Calculate fee based on virtual size (vsize accounts for witness discount)
        let virtual_size = transaction.vsize() as u64;
        if (virtual_size * fee_per_byte) / 1000 == fee {
            // Fee calculation has stabilized
            return transaction;
        } else {
            // Update fee and try again
            fee = (virtual_size * fee_per_byte) / 1000;
        }
    }
}

/// Constructs a reveal transaction with a specific fee amount.
///
/// The reveal transaction has a simple structure:
/// - Input: Spends the commit output from the commit transaction
/// - Output: Sends remaining value (after fee) to the destination address
/// - Witness: Contains the inscription script and proof of authorization
pub fn build_reveal_transaction_with_fee(
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_tx_id: &Txid,
    destination_address: &Address,
    fee: u64,
) -> Result<Transaction, String> {
    // Create input that spends the commit output.
    // The commit transaction created an output at index 0 that we now spend.
    let input = TxIn {
        previous_output: OutPoint {
            txid: commit_tx_id.to_owned(),
            vout: 0, // Output index 0 from commit transaction
        },
        script_sig: ScriptBuf::new(), // Empty for Taproot (uses witness instead)
        sequence: Sequence::MAX,      // No relative timelock constraints
        witness: Witness::new(),      // Will be populated with actual witness data
    };

    // Create output that sends remaining funds (minus fee) to destination.
    // The inscription is now "bound" to these satoshis according to ordinal theory.
    // In production: Ensure the fee is smaller than the output value to avoid
    // underflow scenarios.
    let output = TxOut {
        value: Amount::from_sat(INSCRIPTION_OUTPUT_VALUE - fee),
        script_pubkey: destination_address.script_pubkey(),
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
    transaction.input[0]
        .witness
        .push(SCHNORR_SIGNATURE_PLACEHOLDER);
    transaction.input[0].witness.push(reveal_script.to_bytes());
    transaction.input[0].witness.push(control_block.serialize());

    Ok(transaction)
}

/// Builds the Ordinals reveal script that contains the inscription data.
///
/// The reveal script follows the Ordinals protocol format:
/// - Starts with key verification (internal_key + OP_CHECKSIG)
/// - Contains inscription envelope (OP_FALSE OP_IF ... OP_ENDIF)
/// - The envelope ensures data is in the witness but never executed
pub fn build_ordinal_reveal_script(
    internal_key: &XOnlyPublicKey,
    inscription_payload: &PushBytesBuf,
) -> ScriptBuf {
    Builder::new()
        .push_slice(internal_key.serialize()) // 32-byte x-only key
        .push_opcode(OP_CHECKSIG) // Verify signature for spending
        .push_opcode(OP_FALSE) // Push false to skip inscription data
        .push_opcode(OP_IF) // Begin inscription envelope
        .push_slice(b"ord") // Ordinals protocol marker
        .push_int(1) // Content type field number
        .push_slice(b"text/plain") // MIME type of the inscription
        .push_int(0) // Data push field number
        .push_slice(inscription_payload) // The actual inscription content
        .push_opcode(OP_ENDIF) // End inscription envelope
        .into_script()
}

/// Creates the witness stack for Taproot script-path spending.
///
/// This function handles the complex process of:
/// 1. Computing the signature hash for script-path spending
/// 2. Signing with the appropriate private key
/// 3. Constructing the witness stack in the correct order
///
/// The witness stack must be ordered correctly for script validation:
/// - Signature (top of stack, consumed by OP_CHECKSIG)
/// - Script (the code to execute)
/// - Control block (proves the script is part of the Taproot commitment)
pub async fn create_script_path_witness(
    ctx: &BitcoinContext,
    reveal_transaction: &mut Transaction,
    commit_output: &TxOut,
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    internal_key_path: Vec<Vec<u8>>,
) {
    // Calculate the signature hash for Taproot script-path spending.
    // This is different from key-path spending and commits to the specific script.
    let mut sighash_cache = SighashCache::new(reveal_transaction);
    let leaf_hash = TapLeafHash::from_script(reveal_script, LeafVersion::TapScript);

    // Compute the signature hash that we need to sign.
    // This hash commits to:
    // - The transaction being signed (amounts, outputs, etc.)
    // - The specific script being executed
    // - The input being spent
    // This prevents signature reuse attacks and ensures the signature is only valid
    // for this exact transaction and script.
    let sighash = sighash_cache
        .taproot_script_spend_signature_hash(
            0,                                        // Input index we're signing
            &Prevouts::All(&[commit_output.clone()]), // Previous outputs being spent
            leaf_hash,                                // Hash of the script being executed
            TapSighashType::Default,                  // Sign all inputs and outputs
        )
        .unwrap()
        .as_byte_array()
        .to_vec();

    // Sign the computed hash using our private key.
    // This proves we have the authority to spend these funds according to
    // the rules in our inscription script (which requires a valid signature).
    let schnorr_signature_bytes = sign_with_schnorr(
        ctx.key_name.to_string(), // Key identifier in the IC canister
        internal_key_path,        // Same derivation path as commit tx
        None,                     // No merkle root for script-path signing
        sighash,
    )
    .await;

    // Wrap the raw signature with its sighash type.
    // The sighash type tells verifiers what parts of the transaction this
    // signature commits to (in our case, everything).
    let taproot_script_signature = bitcoin::taproot::Signature {
        signature: bitcoin::secp256k1::schnorr::Signature::from_slice(&schnorr_signature_bytes)
            .unwrap(),
        sighash_type: TapSighashType::Default,
    };

    // Construct the witness stack for script-path spending.
    // The witness contains all data needed to validate the script execution:
    // 1. Signature: Proves we can satisfy the script's OP_CHECKSIG
    // 2. Script: The actual inscription script to execute
    // 3. Control block: Proves this script is committed to by the address
    //
    // When validators process this transaction, they'll execute our script
    // with the signature, revealing our inscription data in the process.
    let witness_stack = sighash_cache.witness_mut(0).unwrap();
    witness_stack.clear();
    witness_stack.push(taproot_script_signature.to_vec());
    witness_stack.push(reveal_script.to_bytes());
    witness_stack.push(control_block.serialize());
}
