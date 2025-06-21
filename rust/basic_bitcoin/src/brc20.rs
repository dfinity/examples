use bitcoin::{
    opcodes::{all::*, OP_FALSE},
    script::{Builder, PushBytesBuf},
    ScriptBuf, XOnlyPublicKey,
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
        .push_slice(b"application/json") // MIME type indicating BRC-20 JSON content
        .push_int(0) // Data field number (standardized in Ordinals protocol)
        .push_slice(&inscription_payload) // The actual BRC-20 JSON token data
        .push_opcode(OP_ENDIF) // End inscription envelope
        .into_script()
}
