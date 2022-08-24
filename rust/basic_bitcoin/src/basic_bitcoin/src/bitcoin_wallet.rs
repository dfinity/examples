//! A demo of a very bare-bones bitcoin "wallet".
//!
//! The wallet here showcases how bitcoin addresses can be be computed
//! and how bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2WPKH.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.
use crate::{bitcoin_api, ecdsa_api};
use bech32::{u5, Variant};
use bitcoin::util::psbt::serialize::Serialize;
use bitcoin::{
    blockdata::witness::Witness, hashes::Hash, util::sighash, Address, AddressType,
    EcdsaSighashType, OutPoint, Script, Transaction, TxIn, TxOut, Txid,
};
use ic_btc_types::{MillisatoshiPerByte, Network, Satoshi, Utxo};
use ic_cdk::print;
use sha2::Digest;
use std::str::FromStr;

const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

/// Returns the P2WPKH address of this canister at the given derivation path.
pub async fn get_p2wpkh_address(
    network: Network,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> String {
    // Fetch the public key of the given derivation path.
    let public_key = ecdsa_api::ecdsa_public_key(key_name, derivation_path).await;

    // Compute the address.
    public_key_to_p2wpkh_address(network, &public_key)
}

/// Sends a transaction to the network that transfers the given amount to the
/// given destination, where the source of the funds is the canister itself
/// at the given derivation path.
pub async fn send(
    network: Network,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = bitcoin_api::get_current_fee_percentiles(network).await;

    let fee_per_byte = if fee_percentiles.is_empty() {
        // There are no fee percentiles. This case can only happen on a regtest
        // network where there are no non-coinbase transactions. In this case,
        // we use a default of 2000 millisatoshis/byte (i.e. 2 satoshi/byte)
        2000
    } else {
        // Choose the 50th percentile for sending fees.
        fee_percentiles[49]
    };

    // Fetch our public key, P2WPKH address, and UTXOs.
    let own_public_key =
        ecdsa_api::ecdsa_public_key(key_name.clone(), derivation_path.clone()).await;
    let own_address = public_key_to_p2wpkh_address(network, &own_public_key);

    print("Fetching UTXOs...");
    let own_utxos = bitcoin_api::get_utxos(network, own_address.clone())
        .await
        .utxos;

    let own_address = Address::from_str(&own_address).unwrap();
    let dst_address = Address::from_str(&dst_address).unwrap();

    // Build the transaction that sends `amount` to the destination address.
    let transaction = build_transaction(
        &own_public_key,
        &own_address,
        &own_utxos,
        &dst_address,
        amount,
        fee_per_byte,
    )
    .await;

    let tx_bytes = transaction.serialize();
    print(&format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = sign_transaction(
        &own_public_key,
        &own_address,
        &own_utxos,
        transaction,
        key_name,
        derivation_path,
        ecdsa_api::sign_with_ecdsa,
    )
    .await;

    let signed_transaction_bytes = signed_transaction.serialize();
    print(&format!(
        "Signed transaction: {}",
        hex::encode(&signed_transaction_bytes)
    ));

    print("Sending transaction...");
    bitcoin_api::send_transaction(network, signed_transaction_bytes).await;
    print("Done");

    signed_transaction.txid()
}

// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
async fn build_transaction(
    own_public_key: &[u8],
    own_address: &Address,
    own_utxos: &[Utxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_vbyte: MillisatoshiPerByte,
) -> Transaction {
    // We have a chicken-and-egg problem where we need to know the length
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    print("Building transaction...");
    let mut total_fee = 0;
    loop {
        let transaction =
            build_transaction_with_fee(own_utxos, own_address, dst_address, amount, total_fee)
                .expect("Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            own_public_key,
            own_address,
            own_utxos,
            transaction.clone(),
            String::from(""), // mock key name
            vec![],           // mock derivation path
            mock_signer,
        )
        .await;

        // The virtual size is a quarter of the transaction weight rounded up.
        let tx_vsize = ((Transaction::weight(&signed_transaction) +3) / 4) as u64;

        if (tx_vsize * fee_per_vbyte) / 1000 == total_fee {
            print(&format!("Transaction built with fee {}.", total_fee));
            return transaction;
        } else {
            total_fee = (tx_vsize * fee_per_vbyte) / 1000;
        }
    }
}

fn build_transaction_with_fee(
    own_utxos: &[Utxo],
    own_address: &Address,
    dst_address: &Address,
    amount: u64,
    fee: u64,
) -> Result<Transaction, String> {
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
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: 0xffffffff,
            witness: Witness::new(),
            script_sig: Script::new(),
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

    for utxo in own_utxos {
        print(&format!("Consuming an output with value: {}", utxo.value));
    }

    Ok(Transaction {
        input: inputs,
        output: outputs,
        lock_time: 0,
        version: 1,
    })
}

fn get_value(input: &TxIn, utxos: &[Utxo]) -> u64 {
    let used_utxo = utxos
        .iter()
        .find(|utxo| {
            Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap())
                == input.previous_output.txid
                && utxo.outpoint.vout == input.previous_output.vout
        })
        .unwrap();
    used_utxo.value
}

// Sign a bitcoin transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2WPKH address.
async fn sign_transaction<SignFun, Fut>(
    own_public_key: &[u8],
    own_address: &Address,
    own_utxos: &[Utxo],
    mut transaction: Transaction,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    // Verify that our own address is P2WPKH.
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2wpkh),
        "This example supports signing p2wpkh addresses only."
    );

    let txclone = transaction.clone();
    let mut hash_cache = sighash::SighashCache::new(&txclone);
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let value = get_value(input, own_utxos);
        print(&format!("Input spent with value: {}", value));
        let sighash = hash_cache
            .segwit_signature_hash(index, &own_address.script_pubkey(), value, SIG_HASH_TYPE)
            .expect("Creating the segwit signature hash failed.");

        let signature = signer(key_name.clone(), derivation_path.clone(), sighash.to_vec()).await;

        // Convert signature to DER.
        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);
        print(&format!("Public key length: {}", own_public_key.len()));
        let witness_bytes = vec![sig_with_hashtype, own_public_key.to_vec()];
        input.witness = Witness::from_vec(witness_bytes);
    }

    transaction
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// Converts a public key to a P2WPKH address.
fn public_key_to_p2wpkh_address(network: Network, public_key: &[u8]) -> String {
    let data: [u8; 20] = ripemd::Ripemd160::digest(sha256(public_key)).into();
    let witness_version: u5 = u5::try_from_u8(0).unwrap();
    let data: Vec<u5> = std::iter::once(witness_version)
        .chain(
            bech32::convert_bits(&data[..], 8, 5, true)
                .unwrap()
                .into_iter()
                .map(|b| u5::try_from_u8(b).unwrap()),
        )
        .collect();
    let hrp = match network {
        ic_btc_types::Network::Mainnet => "bc",   // mainnet
        ic_btc_types::Network::Testnet => "tb",   // testnet
        ic_btc_types::Network::Regtest => "bcrt", // regtest
    };
    bech32::encode(hrp, data, Variant::Bech32).unwrap()
}

// A mock for rubber-stamping ECDSA signatures.
async fn mock_signer(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    vec![1; 64]
}

// Converts a SEC1 ECDSA signature to the DER format.
fn sec1_to_der(sec1_signature: Vec<u8>) -> Vec<u8> {
    let r: Vec<u8> = if sec1_signature[0] & 0x80 != 0 {
        // r is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[..32].to_vec());
        tmp
    } else {
        // r is positive.
        sec1_signature[..32].to_vec()
    };

    let s: Vec<u8> = if sec1_signature[32] & 0x80 != 0 {
        // s is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[32..].to_vec());
        tmp
    } else {
        // s is positive.
        sec1_signature[32..].to_vec()
    };

    // Convert signature to DER.
    vec![
        vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
        r,
        vec![0x02, s.len() as u8],
        s,
    ]
    .into_iter()
    .flatten()
    .collect()
}
