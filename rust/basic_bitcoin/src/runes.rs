// This module implements Bitcoin Runes protocol functionality.
// Runes are a fungible token protocol built on Bitcoin that creates tokens
// using OP_RETURN outputs with OP_13 markers for on-chain metadata storage.

use bitcoin::{
    opcodes::all::*,
    script::{Builder, PushBytesBuf},
    ScriptBuf,
};

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

impl Tag {
    fn into_u128(self) -> u128 {
        match self {
            Tag::Body => 0,
            Tag::Flags => 2,
            Tag::Rune => 4,
            Tag::Premine => 6,
            Tag::Cap => 8,
            Tag::Amount => 10,
            Tag::HeightStart => 12,
            Tag::HeightEnd => 14,
            Tag::OffsetStart => 16,
            Tag::OffsetEnd => 18,
            Tag::Mint => 20,
            Tag::Pointer => 22,
            Tag::Cenotaph => 126,
            Tag::Divisibility => 1,
            Tag::Spacers => 3,
            Tag::Symbol => 5,
            Tag::Nop => 127,
        }
    }
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
pub fn encode_leb128(mut value: u128) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80; // Set continuation bit
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
    bytes
}

/// Encodes a rune name into its numeric representation.
///
/// Runes use a modified base-26 encoding where A=0, B=1, ... Z=25.
/// Names are encoded with A as the least significant digit for compact storage.
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
        payload.extend_from_slice(&encode_leb128(Tag::Divisibility.into_u128()));
        payload.extend_from_slice(&encode_leb128(etching.divisibility as u128));
    }

    // Tag 2: Flags
    payload.extend_from_slice(&encode_leb128(Tag::Flags.into_u128()));
    payload.extend_from_slice(&encode_leb128(flags));

    // Tag 3: Spacers (odd tag)
    if etching.spacers > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Spacers.into_u128()));
        payload.extend_from_slice(&encode_leb128(etching.spacers as u128));
    }

    // Tag 4: Rune name
    payload.extend_from_slice(&encode_leb128(Tag::Rune.into_u128()));
    payload.extend_from_slice(&encode_leb128(encoded_name));

    // Tag 5: Symbol (odd tag)
    if let Some(symbol) = etching.symbol {
        payload.extend_from_slice(&encode_leb128(Tag::Symbol.into_u128()));
        payload.extend_from_slice(&encode_leb128(symbol as u128));
    }

    // Tag 6: Premine
    if etching.premine > 0 {
        payload.extend_from_slice(&encode_leb128(Tag::Premine.into_u128()));
        payload.extend_from_slice(&encode_leb128(etching.premine));
    }

    // Add mint terms if present
    if let Some(terms) = &etching.terms {
        if let Some(amount) = terms.amount {
            payload.extend_from_slice(&encode_leb128(Tag::Amount.into_u128()));
            payload.extend_from_slice(&encode_leb128(amount));
        }
        if let Some(cap) = terms.cap {
            payload.extend_from_slice(&encode_leb128(Tag::Cap.into_u128()));
            payload.extend_from_slice(&encode_leb128(cap));
        }
        if let Some(start) = terms.height.0 {
            payload.extend_from_slice(&encode_leb128(Tag::HeightStart.into_u128()));
            payload.extend_from_slice(&encode_leb128(start as u128));
        }
        if let Some(end) = terms.height.1 {
            payload.extend_from_slice(&encode_leb128(Tag::HeightEnd.into_u128()));
            payload.extend_from_slice(&encode_leb128(end as u128));
        }
        if let Some(start) = terms.offset.0 {
            payload.extend_from_slice(&encode_leb128(Tag::OffsetStart.into_u128()));
            payload.extend_from_slice(&encode_leb128(start as u128));
        }
        if let Some(end) = terms.offset.1 {
            payload.extend_from_slice(&encode_leb128(Tag::OffsetEnd.into_u128()));
            payload.extend_from_slice(&encode_leb128(end as u128));
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
