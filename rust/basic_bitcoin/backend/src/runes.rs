// This module implements Bitcoin Runes protocol functionality.
// Runes are a fungible token protocol built on Bitcoin that creates tokens
// using OP_RETURN outputs with OP_13 markers for on-chain metadata storage.

use bitcoin::{
    opcodes::all::*,
    script::{Builder, PushBytesBuf},
    ScriptBuf,
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
    #[allow(dead_code)]
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

/// Encodes a u128 as LEB128 (Little Endian Base 128).
///
/// The Runes protocol uses LEB128 encoding for all integer values in the runestone
/// to create compact, variable-length representations that minimize transaction size.
pub fn encode_leb128(value: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    write::unsigned(&mut buf, value).unwrap();
    buf
}

/// Encodes a rune name into its numeric representation.
///
/// Runes use a modified base-26 encoding where A=0, B=1, ... Z=25.
/// Names are encoded with A as the least significant digit for compact storage.
pub fn encode_rune_name(name: &str) -> Result<u64, String> {
    if name.is_empty() {
        return Err("Rune name cannot be empty".to_string());
    }

    let mut value = 0u64;
    for (i, ch) in name.chars().enumerate() {
        if i >= 28 {
            return Err("Rune name cannot exceed 28 characters".to_string());
        }
        if !ch.is_ascii_uppercase() {
            return Err("Rune name must contain only uppercase letters A-Z".to_string());
        }

        let digit = (ch as u8 - b'A') as u64;
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
    /// OP_RETURN outputs count toward the index but cannot hold rune balances.
    pub output: u32,
}

/// Builds a runestone script for transferring rune tokens between addresses.
///
/// A transfer runestone contains only edicts — no etching fields. The edicts
/// direct the protocol to move tokens from input UTXOs holding rune balances
/// to specific transaction outputs. Any balance not explicitly allocated by an
/// edict flows to the last non-OP_RETURN output by default.
pub fn build_transfer_script(edicts: &[Edict]) -> Result<ScriptBuf, String> {
    if edicts.is_empty() {
        return Err("At least one edict is required".to_string());
    }

    let mut payload = Vec::new();

    // Tag 0 (Body): separator signalling that edicts follow.
    // A pure transfer runestone has no named fields before this.
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

    // Tag 4: Rune name
    payload.extend_from_slice(&encode_leb128(Tag::Rune as u64));
    payload.extend_from_slice(&encode_leb128(encoded_name as u64));

    // Tag 5: Symbol (odd tag)
    if let Some(symbol) = etching.symbol {
        payload.extend_from_slice(&encode_leb128(Tag::Symbol as u64));
        payload.extend_from_slice(&encode_leb128(symbol as u64));
    }

    // Tag 6: Premine
    if etching.premine > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Premine as u64));
        payload.extend_from_slice(&encode_leb128(etching.premine as u64));
    }

    // Add mint terms if present
    if let Some(terms) = &etching.terms {
        if let Some(amount) = terms.amount {
            payload.extend_from_slice(&encode_leb128(Tag::Amount as u64));
            payload.extend_from_slice(&encode_leb128(amount as u64));
        }
        if let Some(cap) = terms.cap {
            payload.extend_from_slice(&encode_leb128(Tag::Cap as u64));
            payload.extend_from_slice(&encode_leb128(cap as u64));
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
