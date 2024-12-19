use crate::bitcoin_api;
use bitcoin::{
    absolute::LockTime, blockdata::witness::Witness, hashes::Hash, Address, Network, OutPoint,
    ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid,
};
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Utxo};

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
            sequence: Sequence::max_value(),
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect();

    let prevouts = utxos_to_spend
        .into_iter()
        .map(|utxo| TxOut {
            value: utxo.value,
            script_pubkey: own_address.script_pubkey(),
        })
        .collect();

    let mut outputs = vec![TxOut {
        script_pubkey: dst_address.script_pubkey(),
        value: amount,
    }];

    let remaining_amount = total_spent - amount - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: remaining_amount,
        });
    }

    Ok((
        Transaction {
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
            version: 2,
        },
        prevouts,
    ))
}

pub fn transform_network(network: BitcoinNetwork) -> Network {
    match network {
        BitcoinNetwork::Mainnet => Network::Bitcoin,
        BitcoinNetwork::Testnet => Network::Testnet,
        BitcoinNetwork::Regtest => Network::Regtest,
    }
}

pub async fn get_fee_per_byte(network: BitcoinNetwork) -> u64 {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = bitcoin_api::get_current_fee_percentiles(network).await;

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

// A mock for rubber-stamping signatures.
pub async fn mock_signer(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    vec![255; 64]
}
