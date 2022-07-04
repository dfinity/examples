//! A demo of a very bare-bones bitcoin "wallet".
//!
//! The wallet here showcases how bitcoin addresses can be be computed
//! and how bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2PKH.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.
use crate::{bitcoin_api, ecdsa_api};
use bitcoin::util::psbt::serialize::Serialize as _;
use bitcoin::{
    blockdata::script::Builder, hashes::Hash, Address, AddressType, OutPoint, Script, SigHashType,
    Transaction, TxIn, TxOut, Txid,
};
use ic_btc_types::{Network, Satoshi, Utxo};
use ic_cdk::{print, trap};
use sha2::Digest;
use std::str::FromStr;

const SIG_HASH_TYPE: SigHashType = SigHashType::All;

/// Returns the P2PKH address of this canister at the given derivation path.
pub async fn get_p2pkh_address(network: Network, derivation_path: Vec<Vec<u8>>) -> String {
    // Fetch the public key of the given derivation path.
    let public_key = ecdsa_api::ecdsa_public_key(derivation_path).await;

    // Compute the P2PKH address from our public key.
    {
        // sha256 + ripmd160
        let mut hasher = ripemd::Ripemd160::new();
        hasher.update(sha256(public_key));
        let result = hasher.finalize();

        let prefix = match network {
            Network::Testnet | Network::Regtest => 0x6f,
            Network::Mainnet => 0x00,
        };
        let mut data_with_prefix = vec![prefix];
        data_with_prefix.extend(result);

        let checksum = &sha256(sha256(data_with_prefix.clone()))[..4];

        let mut full_address = data_with_prefix;
        full_address.extend(checksum);

        bs58::encode(full_address).into_string()
    }
}

/// Sends a transaction to the network that transfers the given amount to the
/// given destination, where the source of the funds is the canister itself
/// at the given derivation path.
pub async fn send(
    network: Network,
    derivation_path: Vec<Vec<u8>>,
    dst_address: String,
    amount: Satoshi,
) {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = bitcoin_api::get_current_fee_percentiles(network).await;

    // Choose the 75th percentile for sending fees so that the transaction
    // is mined relatively quickly.
    let fee = fee_percentiles[74];

    print!("Fee: {}", fee);

    if amount <= fee {
        trap(&format!(
            "Amount must be higher than the fee of {} satoshis",
            fee,
        ));
    }

    let src_address = get_p2pkh_address(network, derivation_path.clone()).await;

    // Fetch our UTXOs.
    let utxos = bitcoin_api::get_utxos(network, src_address.clone())
        .await
        .utxos;

    let transaction =
        build_transaction(utxos, src_address.clone(), dst_address, amount, fee)
            .expect("Error building transaction.");

    let tx_bytes = transaction.serialize();
    print(&format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction =
        sign_transaction(transaction, src_address, derivation_path).await;

    let signed_transaction_bytes = signed_transaction.serialize();
    print(&format!(
        "Signed transaction: {}",
        hex::encode(signed_transaction_bytes.clone())
    ));

    print("Sending transaction...");
    bitcoin_api::send_transaction(network, signed_transaction_bytes).await;
    print("Done");
}

// Builds a transaction that sends the given `amount` of satoshis to the `destination` address.
fn build_transaction(
    utxos: Vec<Utxo>,
    source: String,
    destination: String,
    amount: u64,
    fee: u64,
) -> Result<Transaction, String> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 10_000;

    let source = Address::from_str(&source).expect("Invalid source address.");
    let destination = Address::from_str(&destination).expect("Invalid destination address.");

    // Select which UTXOs to spend. For now, we naively spend the first available UTXOs,
    // even if they were previously spent in a transaction.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in utxos.into_iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fee {
            // We have enough inputs to cover the amount we want to spend.
            break;
        }
    }

    print(&format!("UTXOs to spend: {:?}", utxos_to_spend));
    print(&format!(
        "UTXO transaction id: {}",
        Txid::from_hash(Hash::from_slice(&utxos_to_spend[0].outpoint.txid).unwrap()),
    ));

    if total_spent < amount {
        return Err("Insufficient balance".to_string());
    }

    let inputs: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: 0xffffffff,
            witness: Vec::new(),
            script_sig: Script::new(),
        })
        .collect();

    let mut outputs = vec![TxOut {
        script_pubkey: destination.script_pubkey(),
        value: amount,
    }];

    let remaining_amount = total_spent - amount - fee;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: source.script_pubkey(),
            value: remaining_amount,
        });
    }

    Ok(Transaction {
        input: inputs,
        output: outputs,
        lock_time: 0,
        version: 2,
    })
}

/// Sign a bitcoin transaction given the private key and the source address of the funds.
///
/// Constraints:
/// * All the inputs are referencing outpoints that are owned by `src_address`.
/// * `src_address` is a P2PKH address.
async fn sign_transaction(
    mut transaction: Transaction,
    src_address: String,
    derivation_path: Vec<Vec<u8>>,
) -> Transaction {
    let src_address = Address::from_str(&src_address).expect("Invalid source address.");

    // Verify that the address is P2PKH. The signature algorithm below is specific to P2PKH.
    assert_eq!(
        src_address.address_type(),
        Some(AddressType::P2pkh),
        "This example supports signing p2pkh addresses only."
    );

    let public_key = ecdsa_api::ecdsa_public_key(derivation_path.clone()).await;

    let txclone = transaction.clone();
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash =
            txclone.signature_hash(index, &src_address.script_pubkey(), SIG_HASH_TYPE.as_u32());

        let signature =
            crate::ecdsa_api::sign_with_ecdsa(derivation_path.clone(), sighash.to_vec()).await;

        // Convert signature to DER.
        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.as_u32() as u8);
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype.as_slice())
            .push_slice(public_key.as_slice())
            .into_script();
        input.witness.clear();
    }

    transaction
}

fn sha256(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
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
