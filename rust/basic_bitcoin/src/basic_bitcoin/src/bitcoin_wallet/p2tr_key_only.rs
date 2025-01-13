use crate::{bitcoin_api, schnorr_api};
use bitcoin::{
    consensus::serialize, key::Secp256k1, secp256k1::PublicKey, taproot::TaprootSpendInfo, Address,
    Txid,
};
use ic_cdk::api::management_canister::bitcoin::{BitcoinNetwork, Satoshi};
use ic_cdk::print;
use std::str::FromStr;

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
    network: BitcoinNetwork,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> Address {
    let public_key = schnorr_api::schnorr_public_key(key_name, derivation_path).await;
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
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public key, P2TR key-only address, and UTXOs.
    let own_public_key =
        schnorr_api::schnorr_public_key(key_name.clone(), derivation_path.clone()).await;
    let x_only_pubkey =
        bitcoin::key::XOnlyPublicKey::from(PublicKey::from_slice(&own_public_key).unwrap());

    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info =
        TaprootSpendInfo::new_key_spend(&secp256k1_engine, x_only_pubkey, None);

    let own_address = Address::p2tr_tweaked(
        taproot_spend_info.output_key(),
        super::common::transform_network(network),
    );

    print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = bitcoin_api::get_utxos(network, own_address.to_string())
        .await
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
    print(format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = super::p2tr::schnorr_sign_key_spend_transaction(
        &own_address,
        transaction,
        prevouts.as_slice(),
        key_name,
        derivation_path,
        vec![],
        schnorr_api::sign_with_schnorr,
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
