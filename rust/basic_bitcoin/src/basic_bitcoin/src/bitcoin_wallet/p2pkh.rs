use crate::{bitcoin_api, ecdsa_api};
use bitcoin::{
    consensus::serialize,
    hashes::Hash,
    script::{Builder, PushBytesBuf},
    sighash::{EcdsaSighashType, SighashCache},
    Address, AddressType, PublicKey, Transaction, Txid,
};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, MillisatoshiPerByte, Satoshi, Utxo,
};
use ic_cdk::print;
use std::convert::TryFrom;
use std::str::FromStr;

use super::common::transform_network;

const ECDSA_SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

/// Returns the P2PKH address of this canister at the given derivation path.
pub async fn get_address(
    network: BitcoinNetwork,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> String {
    // Fetch the public key of the given derivation path.
    let public_key = ecdsa_api::get_ecdsa_public_key(key_name, derivation_path).await;

    // Compute the address.
    public_key_to_p2pkh_address(network, &public_key)
}

/// Sends a transaction to the network that transfers the given amount to the
/// given destination, where the source of the funds is the canister itself
/// at the given derivation path.
pub async fn send(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public key, P2PKH address, and UTXOs.
    let own_public_key =
        ecdsa_api::get_ecdsa_public_key(key_name.clone(), derivation_path.clone()).await;
    let own_address = public_key_to_p2pkh_address(network, &own_public_key);

    print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = bitcoin_api::get_utxos(network, own_address.clone())
        .await
        .utxos;

    let own_address = Address::from_str(&own_address)
        .unwrap()
        .require_network(super::common::transform_network(network))
        .expect("should be valid address for the network");
    let dst_address = Address::from_str(&dst_address)
        .unwrap()
        .require_network(super::common::transform_network(network))
        .expect("should be valid address for the network");

    // Build the transaction that sends `amount` to the destination address.
    let transaction = build_p2pkh_spend_tx(
        &own_public_key,
        &own_address,
        &own_utxos,
        &dst_address,
        amount,
        fee_per_byte,
    )
    .await;

    let tx_bytes = serialize(&transaction);
    print(format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = ecdsa_sign_transaction(
        &own_public_key,
        &own_address,
        transaction,
        key_name,
        derivation_path,
        ecdsa_api::get_ecdsa_signature,
    )
    .await;

    let signed_transaction_bytes = serialize(&signed_transaction);
    print(format!(
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
async fn build_p2pkh_spend_tx(
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
        let (transaction, _prevouts) = super::common::build_transaction_with_fee(
            own_utxos,
            own_address,
            dst_address,
            amount,
            total_fee,
        )
        .expect("Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = ecdsa_sign_transaction(
            own_public_key,
            own_address,
            transaction.clone(),
            String::from(""), // mock key name
            vec![],           // mock derivation path
            super::common::mock_signer,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;

        if (tx_vsize * fee_per_vbyte) / 1000 == total_fee {
            print(format!("Transaction built with fee {}.", total_fee));
            return transaction;
        } else {
            total_fee = (tx_vsize * fee_per_vbyte) / 1000;
        }
    }
}

// Sign a bitcoin transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2PKH address.
async fn ecdsa_sign_transaction<SignFun, Fut>(
    own_public_key: &[u8],
    own_address: &Address,
    mut transaction: Transaction,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    // Verify that our own address is P2PKH.
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2pkh),
        "This example supports signing p2pkh addresses only."
    );

    let txclone = transaction.clone();
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash = SighashCache::new(&txclone)
            .legacy_signature_hash(
                index,
                &own_address.script_pubkey(),
                ECDSA_SIG_HASH_TYPE.to_u32(),
            )
            .unwrap();

        let signature = signer(
            key_name.clone(),
            derivation_path.clone(),
            sighash.as_byte_array().to_vec(),
        )
        .await;

        // Convert signature to DER.
        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype: Vec<u8> = der_signature;
        sig_with_hashtype.push(ECDSA_SIG_HASH_TYPE.to_u32() as u8);

        let sig_with_hashtype_push_bytes = PushBytesBuf::try_from(sig_with_hashtype).unwrap();
        let own_public_key_push_bytes = PushBytesBuf::try_from(own_public_key.to_vec()).unwrap();
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype_push_bytes)
            .push_slice(own_public_key_push_bytes)
            .into_script();
        input.witness.clear();
    }

    transaction
}

// Converts a public key to a P2PKH address.
fn public_key_to_p2pkh_address(network: BitcoinNetwork, public_key: &[u8]) -> String {
    Address::p2pkh(
        &PublicKey::from_slice(public_key).expect("failed to parse public key"),
        transform_network(network),
    )
    .to_string()
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
