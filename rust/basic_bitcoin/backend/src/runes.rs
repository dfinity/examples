// This module implements Bitcoin Runes protocol functionality.
// Runes are a fungible token protocol built on Bitcoin that creates tokens
// using OP_RETURN outputs with OP_13 markers for on-chain metadata storage.

use bitcoin::{
    opcodes::all::*,
    script::{Builder, PushBytesBuf},
    ScriptBuf, XOnlyPublicKey,
};
use leb128::write;

#[allow(dead_code)]
const MAX_DIVISIBILITY: u8 = 38;
#[allow(dead_code)]
const MAX_SPACERS: u32 = 0b00000111111111111111111111111111;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tag {
    #[allow(dead_code)]
    Body = 0,
    Flags = 2,
    Rune = 4,
    Premine = 6,
    Cap = 8,
    Amount = 10,
    HeightStart = 12,
    HeightEnd = 14,
    OffsetStart = 16,
    OffsetEnd = 18,
    #[allow(dead_code)]
    Mint = 20,
    Pointer = 22,
    #[allow(dead_code)]
    Cenotaph = 126,
    // Odd tags
    Divisibility = 1,
    Spacers = 3,
    Symbol = 5,
    #[allow(dead_code)]
    Nop = 127,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flag {
    Etching = 0,
    Terms = 1,
    Turbo = 2,
    #[allow(dead_code)]
    Cenotaph = 127,
}

impl Flag {
    fn mask(self) -> u128 {
        let position = match self {
            Flag::Etching => 0,
            Flag::Terms => 1,
            Flag::Turbo => 2,
            Flag::Cenotaph => 127,
        };
        1 << position
    }
}

/// Encodes a u64 as LEB128 (Little Endian Base 128).
///
/// The Runes protocol uses LEB128 encoding for all integer values in the runestone
/// to create compact, variable-length representations that minimize transaction size.
pub fn encode_leb128(value: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    write::unsigned(&mut buf, value).unwrap();
    buf
}

/// Encodes a u128 as LEB128. Used for rune names, premine, amount, and cap fields,
/// which are all u128 in the Runes protocol specification.
pub fn encode_leb128_u128(mut value: u128) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            buf.push(byte);
            break;
        } else {
            buf.push(byte | 0x80);
        }
    }
    buf
}

/// Encodes a rune name into its numeric representation as u128.
///
/// Runes use a modified base-26 encoding where A=0, B=1, ... Z=25.
/// Names are encoded with A as the least significant digit for compact storage.
/// The result is u128 because names of 14+ characters can exceed u64::MAX.
pub fn encode_rune_name(name: &str) -> Result<u128, String> {
    if name.is_empty() {
        return Err("Rune name cannot be empty".to_string());
    }

    let mut value = 0u128;
    for (i, ch) in name.chars().enumerate() {
        if i >= 28 {
            return Err("Rune name cannot exceed 28 characters".to_string());
        }
        if !ch.is_ascii_uppercase() {
            return Err("Rune name must contain only uppercase letters A-Z".to_string());
        }

        let digit = (ch as u8 - b'A') as u128;
        if i == 0 {
            value = digit;
        } else {
            // Multiply previous value by 26 and add new digit
            value = value
                .checked_add(1)
                .and_then(|v| v.checked_mul(26))
                .and_then(|v| v.checked_add(digit))
                .ok_or("Rune name value overflow")?;
        }
    }

    Ok(value)
}

/// Represents a rune etching configuration.
///
/// An etching defines all the parameters for creating a new rune token,
/// including supply, divisibility, and minting terms.
pub struct Etching {
    pub divisibility: u8,
    pub premine: u128,
    pub rune_name: String,
    pub symbol: Option<char>,
    pub terms: Option<Terms>,
    pub turbo: bool,
    pub spacers: u32,
}

/// Defines the terms for open minting of rune tokens.
///
/// Terms specify when and how much additional supply can be minted
/// after the initial etching through public mint operations.
pub struct Terms {
    pub amount: Option<u128>,               // Amount per mint
    pub cap: Option<u128>,                  // Maximum number of mints
    pub height: (Option<u64>, Option<u64>), // Absolute block height range
    pub offset: (Option<u64>, Option<u64>), // Relative block height range
}

/// An edict in a Runestone that transfers rune tokens to a specific transaction output.
///
/// Edicts describe how to allocate rune balances among transaction outputs.
/// The rune ID (block + tx) uniquely identifies which rune to transfer.
pub struct Edict {
    /// Block height where the rune was etched (first component of rune ID).
    pub rune_id_block: u64,
    /// Transaction index within that block (second component of rune ID).
    pub rune_id_tx: u32,
    /// Number of rune tokens to move to the specified output.
    pub amount: u64,
    /// Index into the transaction's vout array that receives the rune tokens.
    /// All vouts (including OP_RETURN) count toward the index. Pointing an edict
    /// at an OP_RETURN output or a non-existent output burns those tokens.
    pub output: u32,
}

/// Builds a runestone script for transferring rune tokens between addresses.
///
/// A transfer runestone contains only edicts — no etching fields. The edicts
/// direct the protocol to move tokens from input UTXOs holding rune balances
/// to specific transaction outputs. Any balance not explicitly allocated by an
/// edict flows to the first non-OP_RETURN output by default.
///
/// `pointer`, if set, overrides the default: unallocated runes go to the output
/// at that vout index instead of the first non-OP_RETURN output. This is used to
/// direct change rune tokens to the change output (vout[2]) when the recipient
/// is at vout[0] and the OP_RETURN is at vout[1].
pub fn build_transfer_script(edicts: &[Edict], pointer: Option<u32>) -> Result<ScriptBuf, String> {
    if edicts.is_empty() {
        return Err("At least one edict is required".to_string());
    }

    let mut payload = Vec::new();

    // Named fields must appear before the Body (0) separator.
    // Tag 22: Pointer — vout index that receives unallocated runes.
    // Without this, unallocated runes would default to the first non-OP_RETURN
    // output (vout[0], the recipient), incorrectly giving them the full balance.
    if let Some(p) = pointer {
        payload.extend_from_slice(&encode_leb128(Tag::Pointer as u64));
        payload.extend_from_slice(&encode_leb128(p as u64));
    }

    // Tag 0 (Body): separator signalling that edicts follow.
    payload.extend_from_slice(&encode_leb128(Tag::Body as u64));

    // Edicts are encoded as groups of 4 LEB128 integers: [block, tx, amount, output].
    // Block and tx use delta encoding relative to the previous edict's rune ID.
    let mut prev_block: u64 = 0;
    let mut prev_tx: u32 = 0;
    for edict in edicts {
        let block_delta = edict.rune_id_block - prev_block;
        // When the block delta is 0 (same block), tx is a delta from the previous tx.
        // When the block delta is non-zero (different block), tx is an absolute value.
        let tx_encoded = if block_delta == 0 {
            (edict.rune_id_tx - prev_tx) as u64
        } else {
            edict.rune_id_tx as u64
        };

        payload.extend_from_slice(&encode_leb128(block_delta));
        payload.extend_from_slice(&encode_leb128(tx_encoded));
        payload.extend_from_slice(&encode_leb128(edict.amount));
        payload.extend_from_slice(&encode_leb128(edict.output as u64));

        prev_block = edict.rune_id_block;
        prev_tx = edict.rune_id_tx;
    }

    let mut builder = Builder::new().push_opcode(OP_RETURN);
    builder = builder.push_opcode(OP_PUSHNUM_13);

    let mut push_bytes = PushBytesBuf::new();
    push_bytes
        .extend_from_slice(&payload)
        .map_err(|_| "Failed to create push bytes - payload may be too large")?;
    builder = builder.push_slice(&push_bytes);

    Ok(builder.into_script())
}

/// Computes the commitment bytes for a rune name.
///
/// The commitment is the rune's numeric encoding as a u128 value in little-endian byte order
/// with trailing zero bytes stripped. The ord indexer scans the commit tapscript for a pushdata
/// instruction equal to these bytes to validate that the etching transaction committed to the name.
pub fn build_rune_commitment_bytes(name: &str) -> Result<Vec<u8>, String> {
    let encoded = encode_rune_name(name)?;
    let bytes = encoded.to_le_bytes();
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1] == 0 {
        end -= 1;
    }
    Ok(bytes[..end].to_vec())
}

/// Builds the tapscript for a rune commit transaction.
///
/// The commit script embeds the rune commitment bytes as a pushdata instruction so that
/// the ord indexer can find them when validating the etching. Script structure:
///
///   <commitment_bytes> OP_DROP <internal_key> OP_CHECKSIG
///
/// Spending this output via script-path places the tapscript in the witness, satisfying
/// the ord protocol requirement that etching inputs commit to the rune name.
pub fn build_rune_commit_script(internal_key: &XOnlyPublicKey, name: &str) -> Result<ScriptBuf, String> {
    let commitment = build_rune_commitment_bytes(name)?;
    let mut push_buf = PushBytesBuf::new();
    push_buf
        .extend_from_slice(&commitment)
        .map_err(|e| e.to_string())?;
    Ok(Builder::new()
        .push_slice(&push_buf)
        .push_opcode(OP_DROP)
        .push_slice(internal_key.serialize())
        .push_opcode(OP_CHECKSIG)
        .into_script())
}

/// Builds a runestone script for etching a new rune token.
///
/// The runestone is encoded as an OP_RETURN output with the format:
/// OP_RETURN OP_13 [LEB128 encoded tag-value pairs...]
///
/// All rune metadata is encoded as alternating tags and values using LEB128,
/// creating a compact binary representation of the token parameters.
pub fn build_etching_script(etching: &Etching) -> Result<ScriptBuf, String> {
    let mut payload = Vec::new();

    // Encode rune name into numeric format for storage.
    let encoded_name = encode_rune_name(&etching.rune_name)?;

    // Build flags bitmask to indicate which features are enabled.
    let mut flags = Flag::Etching.mask(); // Mark this as an etching operation
    if etching.terms.is_some() {
        flags |= Flag::Terms.mask();
    }
    if etching.turbo {
        flags |= Flag::Turbo.mask();
    }

    // Tag 1: Divisibility (odd tag)
    if etching.divisibility > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Divisibility as u64));
        payload.extend_from_slice(&encode_leb128(etching.divisibility as u64));
    }

    // Tag 2: Flags
    payload.extend_from_slice(&encode_leb128(Tag::Flags as u64));
    payload.extend_from_slice(&encode_leb128(flags as u64));

    // Tag 3: Spacers (odd tag)
    if etching.spacers > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Spacers as u64));
        payload.extend_from_slice(&encode_leb128(etching.spacers as u64));
    }

    // Tag 4: Rune name (u128 — names ≥14 chars exceed u64)
    payload.extend_from_slice(&encode_leb128(Tag::Rune as u64));
    payload.extend_from_slice(&encode_leb128_u128(encoded_name));

    // Tag 5: Symbol (odd tag)
    if let Some(symbol) = etching.symbol {
        payload.extend_from_slice(&encode_leb128(Tag::Symbol as u64));
        payload.extend_from_slice(&encode_leb128(symbol as u64));
    }

    // Tag 6: Premine (u128 — protocol supports values beyond u64)
    if etching.premine > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Premine as u64));
        payload.extend_from_slice(&encode_leb128_u128(etching.premine));
    }

    // Add mint terms if present
    if let Some(terms) = &etching.terms {
        if let Some(amount) = terms.amount {
            payload.extend_from_slice(&encode_leb128(Tag::Amount as u64));
            payload.extend_from_slice(&encode_leb128_u128(amount));
        }
        if let Some(cap) = terms.cap {
            payload.extend_from_slice(&encode_leb128(Tag::Cap as u64));
            payload.extend_from_slice(&encode_leb128_u128(cap));
        }
        if let Some(start) = terms.height.0 {
            payload.extend_from_slice(&encode_leb128(Tag::HeightStart as u64));
            payload.extend_from_slice(&encode_leb128(start));
        }
        if let Some(end) = terms.height.1 {
            payload.extend_from_slice(&encode_leb128(Tag::HeightEnd as u64));
            payload.extend_from_slice(&encode_leb128(end));
        }
        if let Some(start) = terms.offset.0 {
            payload.extend_from_slice(&encode_leb128(Tag::OffsetStart as u64));
            payload.extend_from_slice(&encode_leb128(start));
        }
        if let Some(end) = terms.offset.1 {
            payload.extend_from_slice(&encode_leb128(Tag::OffsetEnd as u64));
            payload.extend_from_slice(&encode_leb128(end));
        }
    }

    // Build the OP_RETURN script
    let mut builder = Builder::new().push_opcode(OP_RETURN);

    // Add OP_13 marker
    builder = builder.push_opcode(OP_PUSHNUM_13);

    // Add the entire payload as a single data push.
    // Critical: All runestone data must be in one push after OP_13,
    // not split into multiple chunks, per the Runes protocol specification.
    let mut push_bytes = PushBytesBuf::new();
    push_bytes
        .extend_from_slice(&payload)
        .map_err(|_| "Failed to create push bytes - payload may be too large")?;
    builder = builder.push_slice(&push_bytes);

    Ok(builder.into_script())
}
