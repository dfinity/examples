use bitcoin::{
    consensus::serialize, key::Secp256k1, secp256k1::PublicKey, taproot::TaprootSpendInfo, Address,
    Txid,
};
use ic_cdk::{
    api::debug_print,
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, Network, Satoshi,
        SendTransactionRequest,
    },
};
use std::str::FromStr;

use crate::schnorr::{schnorr_public_key, sign_with_schnorr};

/// Returns the P2TR key-only address of this canister at the given derivation
/// path.
///
/// Quoting the `bitcoin` crate's rustdoc:
///
/// *Note*: As per BIP341
///
/// When the Merkle root is [`None`], the output key commits to an unspendable script path
/// instead of having no script path. This is achieved by computing the output key point as
/// `Q = P + int(hashTapTweak(bytes(P)))G`. See also [`TaprootSpendInfo::tap_tweak`].
pub async fn get_address(
    network: Network,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> Address {
    let public_key = schnorr_public_key(key_name, derivation_path).await;
    let x_only_pubkey =
        bitcoin::key::XOnlyPublicKey::from(PublicKey::from_slice(&public_key).unwrap());
    let secp256k1_engine = Secp256k1::new();
    Address::p2tr(
        &secp256k1_engine,
        x_only_pubkey,
        None,
        super::common::transform_network(network),
    )
}

/// Sends a P2TR key-only transaction to the network that transfers the
/// given amount to the given destination, where the source of the funds is the
/// canister itself at the given derivation path.
pub async fn send(
    network: Network,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public key, P2TR key-only address, and UTXOs.
    let own_public_key = schnorr_public_key(key_name.clone(), derivation_path.clone()).await;
    let x_only_pubkey =
        bitcoin::key::XOnlyPublicKey::from(PublicKey::from_slice(&own_public_key).unwrap());

    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info =
        TaprootSpendInfo::new_key_spend(&secp256k1_engine, x_only_pubkey, None);

    let own_address = Address::p2tr_tweaked(
        taproot_spend_info.output_key(),
        super::common::transform_network(network),
    );

    debug_print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    let dst_address = Address::from_str(&dst_address)
        .unwrap()
        .require_network(super::common::transform_network(network))
        .expect("should be valid address for the network");
    // Build the transaction that sends `amount` to the destination address.
    let (transaction, prevouts) =
        super::p2tr::build_p2tr_tx(&own_address, &own_utxos, &dst_address, amount, fee_per_byte)
            .await;

    let tx_bytes = serialize(&transaction);
    debug_print(format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = super::p2tr::schnorr_sign_key_spend_transaction(
        &own_address,
        transaction,
        prevouts.as_slice(),
        key_name,
        derivation_path,
        vec![],
        sign_with_schnorr,
    )
    .await;

    let signed_transaction_bytes = serialize(&signed_transaction);
    debug_print(format!(
        "Signed transaction: {}",
        hex::encode(&signed_transaction_bytes)
    ));

    debug_print("Sending transaction...");
    bitcoin_send_transaction(&SendTransactionRequest {
        network,
        transaction: signed_transaction_bytes,
    })
    .await
    .unwrap();
    debug_print("Done");

    signed_transaction.compute_txid()
}
