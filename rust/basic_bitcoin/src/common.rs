// This module provides common utilities for Bitcoin transaction construction and management.
// It includes UTXO selection algorithms, transaction building, fee estimation, and
// BIP-32 derivation path handling used across all Bitcoin address types.

use crate::BitcoinContext;
use bitcoin::{
    self, absolute::LockTime, blockdata::witness::Witness, hashes::Hash, transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid,
};
use ic_cdk::bitcoin_canister::{
    bitcoin_get_current_fee_percentiles, GetCurrentFeePercentilesRequest, Utxo,
};
use std::fmt;

/// Selects UTXOs using a greedy algorithm to cover the required amount plus fee.
///
/// This function iterates through UTXOs in reverse order (oldest last) and accumulates
/// them until the total value covers the payment amount plus transaction fee.
/// This approach helps consolidate older UTXOs and can reduce wallet fragmentation.
///
/// Returns an error if the total UTXO value is insufficient to cover the payment and fee.
pub fn select_utxos_greedy(
    own_utxos: &[Utxo],
    amount: u64,
    fee: u64,
) -> Result<Vec<&Utxo>, String> {
    // Greedily select UTXOs in reverse order (oldest last) until we cover amount + fee.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in own_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            break;
        }
    }

    // Abort if we can't cover the payment + fee.
    if total_spent < amount + fee {
        return Err(format!(
            "Insufficient balance: {}, trying to transfer {} satoshi with fee {}",
            total_spent, amount, fee
        ));
    }

    Ok(utxos_to_spend)
}

/// Selects a single UTXO that can cover the required amount plus fee.
///
/// This function is used when you need to tie a specific operation to a single UTXO,
/// such as with Bitcoin inscriptions where the asset must be associated with specific
/// satoshis. It searches for the first UTXO (in reverse order) that has sufficient value.
///
/// Returns an error if no single UTXO has enough value to cover the payment and fee.
pub fn select_one_utxo(own_utxos: &[Utxo], amount: u64, fee: u64) -> Result<Vec<&Utxo>, String> {
    for utxo in own_utxos.iter().rev() {
        if utxo.value >= amount + fee {
            return Ok(vec![&utxo]);
        }
    }

    Err(format!(
        "No sufficiently large utxo found: amount {} satoshi, fee {}",
        amount, fee
    ))
}

/// Represents the primary output type for a Bitcoin transaction.
///
/// This enum allows transaction builders to specify whether they want to send
/// bitcoin to an address (normal payment) or embed data using OP_RETURN (for
/// protocols like Runes that store metadata on-chain).
pub enum PrimaryOutput {
    /// Pay someone (spendable output).
    Address(Address, u64), // destination address, amount in satoshis
    /// Embed data (unspendable OP_RETURN output).
    OpReturn(ScriptBuf), // script already starts with OP_RETURN
}

/// Constructs a Bitcoin transaction from the given UTXOs and primary output specification.
///
/// This function handles the common pattern of Bitcoin transaction construction:
/// 1. Creates inputs from the selected UTXOs
/// 2. Creates the primary output (payment or OP_RETURN data)
/// 3. Adds a change output if the remainder exceeds the dust threshold
/// 4. Returns both the unsigned transaction and previous outputs needed for signing
///
/// The change output is sent back to `own_address` to prevent value loss, but only
/// if the change amount is above the dust threshold to avoid creating uneconomical outputs.
///
/// Returns the constructed unsigned transaction and the list of previous outputs (`prevouts`)
/// used for signing different address types (P2WPKH, P2TR, etc.).
///
/// Assumes that:
/// - Inputs are unspent and valid (caller's responsibility)
/// - Dust threshold is 1,000 satoshis (outputs below this are omitted)
/// - UTXOs are already filtered to be spendable (confirmed, mature, etc.)
pub fn build_transaction_with_fee(
    utxos_to_spend: Vec<&Utxo>,
    own_address: &Address,
    primary_output: &PrimaryOutput,
    fee: u64,
) -> Result<(Transaction, Vec<TxOut>), String> {
    // Define a dust threshold below which change outputs are discarded.
    // This prevents creating outputs that cost more to spend than they're worth.
    const DUST_THRESHOLD: u64 = 1_000;

    // --- Build Inputs ---
    // Convert UTXOs into transaction inputs, preparing them for signing.
    let inputs: Vec<TxIn> = utxos_to_spend
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::MAX,      // No relative timelock constraints
            witness: Witness::new(),      // Will be filled in during signing
            script_sig: ScriptBuf::new(), // Empty for SegWit and Taproot (uses witness)
        })
        .collect();

    // --- Create Previous Outputs ---
    // Each TxOut represents an output from previous transactions being spent.
    // This data is required for signing P2WPKH and P2TR transactions.
    let prevouts = utxos_to_spend
        .clone()
        .into_iter()
        .map(|utxo| TxOut {
            value: Amount::from_sat(utxo.value),
            script_pubkey: own_address.script_pubkey(),
        })
        .collect();

    // --- Build Outputs ---
    // Create the primary output based on the operation type.
    let mut outputs = Vec::<TxOut>::new();

    match primary_output {
        PrimaryOutput::Address(addr, amt) => outputs.push(TxOut {
            script_pubkey: addr.script_pubkey(),
            value: Amount::from_sat(*amt),
        }),
        PrimaryOutput::OpReturn(script) => outputs.push(TxOut {
            script_pubkey: script.clone(),
            value: Amount::from_sat(0), // OP_RETURN outputs carry no bitcoin value
        }),
    }

    // Calculate change and add change output if above dust threshold.
    // This prevents value loss while avoiding uneconomical outputs.
    let total_in: u64 = utxos_to_spend.iter().map(|u| u.value).sum();
    let change = total_in
        .checked_sub(outputs.iter().map(|o| o.value.to_sat()).sum::<u64>() + fee)
        .ok_or("fee exceeds inputs")?;

    if change >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: Amount::from_sat(change),
        });
    }

    // --- Assemble Transaction ---
    // Create the final unsigned transaction with version 2 for modern features.
    Ok((
        Transaction {
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO, // No absolute timelock
            version: Version::TWO,     // Standard for modern Bitcoin transactions
        },
        prevouts,
    ))
}

/// Estimates a reasonable fee rate for Bitcoin transactions based on network conditions.
///
/// This function queries the Bitcoin network for recent fee percentiles and returns
/// the median (50th percentile) fee rate, which provides a good balance between
/// confirmation time and cost. The fee rate is returned in millisatoshis per byte.
///
/// On regtest networks (local development), fee data is typically unavailable since
/// there are no standard transactions, so the function falls back to a static rate
/// of 2,000 millisatoshis/vbyte (2 sat/vB) which is reasonable for testing.
///
/// # Returns
/// Fee rate in millisatoshis per byte (1,000 msat = 1 satoshi).
pub async fn get_fee_per_byte(ctx: &BitcoinContext) -> u64 {
    // Query recent fee percentiles from the Bitcoin network.
    // This gives us real-time fee data based on recent transaction activity.
    let fee_percentiles = bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest {
        network: ctx.network,
    })
    .await
    .unwrap();

    if fee_percentiles.is_empty() {
        // Empty percentiles indicate that we're likely on regtest with no standard transactions.
        // Use a reasonable fallback that works for development and testing.
        2000 // 2 sat/vB in millisatoshis
    } else {
        // Use the 50th percentile (median) for balanced confirmation time and cost.
        // This avoids both overpaying (high percentiles) and slow confirmation (low percentiles).
        fee_percentiles[50]
    }
}

/// Purpose field for BIP-32 hierarchical deterministic wallet derivation paths.
///
/// The purpose field determines the address type according to Bitcoin Improvement Proposals:
/// - BIP-44 for P2PKH (legacy addresses): purpose = 44'
/// - BIP-84 for P2WPKH (native SegWit addresses): purpose = 84'
/// - BIP-86 for P2TR (Taproot addresses): purpose = 86'
///
/// These standards ensure wallet compatibility and predictable address generation.
pub enum Purpose {
    P2PKH,  // BIP-44
    P2WPKH, // BIP-84
    P2TR,   // BIP-86
}

impl Purpose {
    fn to_u32(&self) -> u32 {
        match self {
            Purpose::P2PKH => 44,
            Purpose::P2WPKH => 84,
            Purpose::P2TR => 86,
        }
    }
}

/// Represents a complete BIP-32 hierarchical deterministic wallet derivation path.
///
/// The path follows the standard format: m / purpose / coin_type / account / change / address_index
/// This structure enables:
/// - Deterministic key generation from a single seed
/// - Logical separation of different address types and accounts
/// - Privacy through address rotation within accounts
///
/// This implementation supports BIP-44 (P2PKH), BIP-84 (P2WPKH), and BIP-86 (P2TR) standards
/// and provides serialization compatible with the Internet Computer's key derivation APIs.
///
/// The concept of a wallet derivation path being hardened does not apply on ICP, since key
/// derivation is entirely handled by the subnet and private keys are never accessible. Derivation paths
/// function purely as deterministic identifiers.
pub struct DerivationPath {
    /// Purpose according to BIP-43 (e.g., 44 for legacy, 84 for SegWit, 86 for Taproot)
    purpose: Purpose,

    /// Coin type (0 = Bitcoin mainnet/testnet). Can be extended for altcoins.
    coin_type: u32,

    /// Logical account identifier. Use this to separate multiple user accounts or roles.
    account: u32,

    /// Chain: 0 = external (receive), 1 = internal (change)
    change: u32,

    /// Address index: used to derive multiple addresses within the same account.
    address_index: u32,
}

impl DerivationPath {
    /// Constructs a new derivation path with the specified parameters.
    ///
    /// Parameters:
    /// - `purpose`: Determines the address type and BIP standard to follow
    /// - `account`: Logical account separation (use different accounts for different users/purposes)
    /// - `address_index`: Address index within the account (increment for new addresses)
    ///
    /// Fixed values:
    /// - `coin_type`: Always 0 (Bitcoin mainnet/testnet)
    /// - `change`: Always 0 (external/receiving addresses, not internal change addresses)
    fn new(purpose: Purpose, account: u32, address_index: u32) -> Self {
        Self {
            purpose,
            coin_type: 0,
            account,
            change: 0,
            address_index,
        }
    }

    /// Convenience constructor for P2PKH (legacy) addresses.
    pub fn p2pkh(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2PKH, account, address_index)
    }

    /// Convenience constructor for P2WPKH (native SegWit) addresses.
    pub fn p2wpkh(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2WPKH, account, address_index)
    }

    /// Convenience constructor for P2TR (Taproot) addresses.
    pub fn p2tr(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2TR, account, address_index)
    }

    /// Converts the derivation path to the binary format expected by IC's key derivation APIs.
    ///
    /// Returns a Vec<Vec<u8>> where each inner Vec represents one level of the path
    /// as a 4-byte big-endian encoded integer.
    pub fn to_vec_u8_path(&self) -> Vec<Vec<u8>> {
        vec![
            self.purpose.to_u32().to_be_bytes().to_vec(),
            self.coin_type.to_be_bytes().to_vec(),
            self.account.to_be_bytes().to_vec(),
            self.change.to_be_bytes().to_vec(),
            self.address_index.to_be_bytes().to_vec(),
        ]
    }
}

impl fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "m/{}/{}/{}/{}/{}",
            self.purpose.to_u32(),
            self.coin_type,
            self.account,
            self.change,
            self.address_index
        )
    }
}
