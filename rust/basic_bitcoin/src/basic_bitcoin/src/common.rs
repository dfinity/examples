use crate::BitcoinContext;
use bitcoin::{
    self, absolute::LockTime, blockdata::witness::Witness, hashes::Hash, transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid,
};
use ic_cdk::bitcoin_canister::{
    bitcoin_get_current_fee_percentiles, GetCurrentFeePercentilesRequest, Utxo,
};
use std::fmt;

pub fn build_transaction_with_fee(
    own_utxos: &[Utxo],
    own_address: &Address,
    dst_address: &Address,
    amount: u64,
    fee: u64,
) -> Result<(Transaction, Vec<TxOut>), String> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 1_000;

    // Select which UTXOs to spend. We naively spend the oldest available UTXOs,
    // even if they were previously spent in a transaction. This isn't a
    // problem as long as at most one transaction is created per block and
    // we're using min_confirmations of 1.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in own_utxos.iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            // We have enough inputs to cover the amount we want to spend.
            break;
        }
    }

    if total_spent < amount + fee {
        return Err(format!(
            "Insufficient balance: {}, trying to transfer {} satoshi with fee {}",
            total_spent, amount, fee
        ));
    }

    let inputs: Vec<TxIn> = utxos_to_spend
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::MAX,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect();

    let prevouts = utxos_to_spend
        .into_iter()
        .map(|utxo| TxOut {
            value: Amount::from_sat(utxo.value),
            script_pubkey: own_address.script_pubkey(),
        })
        .collect();

    let mut outputs = vec![TxOut {
        script_pubkey: dst_address.script_pubkey(),
        value: Amount::from_sat(amount),
    }];

    let change = total_spent - amount - fee;

    if change >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: Amount::from_sat(change),
        });
    }

    Ok((
        Transaction {
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
            version: Version::TWO,
        },
        prevouts,
    ))
}

pub async fn get_fee_per_byte(ctx: &BitcoinContext) -> u64 {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest {
        network: ctx.network,
    })
    .await
    .unwrap();

    if fee_percentiles.is_empty() {
        // There are no fee percentiles. This case can only happen on a regtest
        // network where there are no non-coinbase transactions. In this case,
        // we use a default of 2000 millisatoshis/byte (i.e. 2 satoshi/byte)
        2000
    } else {
        // Choose the 50th percentile for sending fees.
        fee_percentiles[50]
    }
}

/// Purpose field for a BIP-43/44-style derivation path.
/// Determines the address type. Values are defined by:
/// - BIP-44 for P2PKH (legacy): 44'
/// - BIP-84 for P2WPKH (native SegWit): 84'
/// - BIP-86 for P2TR (Taproot): 86'
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

/// Represents a full BIP-32-compatible derivation path:
/// m / purpose' / coin_type' / account' / change / address_index
///
/// This abstraction is suitable for BIP-44 (legacy), BIP-84 (SegWit), and BIP-86 (Taproot),
/// and provides convenience constructors and binary serialization for use with ECDSA/Schnorr
/// key derivation APIs (such as ICP's management canister).
pub struct DerivationPath {
    /// Purpose according to BIP-43 (e.g., 44 for legacy, 84 for SegWit, 86 for Taproot)
    purpose: Purpose,

    /// Coin type (0 = Bitcoin mainnet/testnet). Can be extended for altcoins.
    coin_type: u32,

    /// Logical account identifier. Use this to separate multiple user accounts or roles.
    account: u32,

    /// Chain: 0 = external (receive), 1 = internal (change)
    change: u32,

    /// Address index: used to derive multiple addresses within the same account/chain.
    address_index: u32,
}

impl DerivationPath {
    /// Constructs a new derivation path using the given purpose, account, and address index.
    ///
    /// - `purpose`: Determines the address type (BIP-44 for P2PKH, BIP-84 for P2WPKH, BIP-86 for P2TR).
    /// - `account`: Use to separate logical users or wallets. For multi-user wallets, assign each user a unique account number.
    /// - `address_index`: Used to derive multiple addresses within the same account and chain (e.g., receiving addresses).
    ///
    /// The coin type is set to 0 (Bitcoin), and change is set to 0 (external chain).
    fn new(purpose: Purpose, account: u32, address_index: u32) -> Self {
        Self {
            purpose,
            coin_type: 0,
            account,
            change: 0,
            address_index,
        }
    }

    /// Convenience constructor for p2pkh (legacy) addresses.
    pub fn p2pkh(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2PKH, account, address_index)
    }

    /// Convenience constructor for p2wpkh (native SegWit) addresses.
    pub fn p2wpkh(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2WPKH, account, address_index)
    }

    /// Convenience constructor for p2tr (Taproot) addresses.
    pub fn p2tr(account: u32, address_index: u32) -> Self {
        Self::new(Purpose::P2TR, account, address_index)
    }

    const HARDENED_OFFSET: u32 = 0x8000_0000;

    /// Returns the derivation path as a Vec<Vec<u8>> (one 4-byte big-endian element per level),
    /// suitable for use with the Internet Computer's ECDSA/Schnorr APIs.
    pub fn to_vec_u8_path(&self) -> Vec<Vec<u8>> {
        vec![
            (self.purpose.to_u32() | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            (self.coin_type | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            (self.account | Self::HARDENED_OFFSET)
                .to_be_bytes()
                .to_vec(),
            self.change.to_be_bytes().to_vec(),
            self.address_index.to_be_bytes().to_vec(),
        ]
    }
}

impl fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "m/{}'/{}'/{}'/{}/{}",
            self.purpose.to_u32(),
            self.coin_type,
            self.account,
            self.change,
            self.address_index
        )
    }
}
