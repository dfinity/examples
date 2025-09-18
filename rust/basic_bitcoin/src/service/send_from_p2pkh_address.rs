use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    ecdsa::{get_ecdsa_public_key, sign_with_ecdsa},
    p2pkh::{self},
    SendRequest, BTC_CONTEXT,
};
use bitcoin::{consensus::serialize, Address, PublicKey};
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    },
    trap, update,
};
use std::str::FromStr;

/// Sends the given amount of bitcoin from this smart contract's P2PKH address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2pkh_address(request: SendRequest) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if request.amount_in_satoshi == 0 {
        trap("Amount must be greater than 0");
    }

    // Parse and validate the destination address. The address type needs to be
    // valid for the Bitcoin network we are on.
    let dst_address = Address::from_str(&request.destination_address)
        .unwrap()
        .require_network(ctx.bitcoin_network)
        .unwrap();

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair. To generate a user-specific address,
    // you would typically use a derivation path based on the user's identity or some other unique identifier.
    let derivation_path = DerivationPath::p2pkh(0, 0);

    // Get the ECDSA public key of this smart contract at the given derivation path.
    let own_public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;

    // Convert the public key to the format used by the Bitcoin library.
    let own_public_key = PublicKey::from_slice(&own_public_key).unwrap();

    // Generate a P2PKH address from the public key.
    let own_address = Address::p2pkh(own_public_key, ctx.bitcoin_network);

    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Build the transaction.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let transaction = p2pkh::build_transaction(
        &ctx,
        &own_public_key,
        &own_address,
        &own_utxos,
        &PrimaryOutput::Address(dst_address, request.amount_in_satoshi),
        fee_per_byte,
    )
    .await;

    // Sign the transaction.
    let signed_transaction = p2pkh::sign_transaction(
        &ctx,
        &own_public_key,
        &own_address,
        transaction,
        derivation_path.to_vec_u8_path(),
        sign_with_ecdsa,
    )
    .await;

    // Send the transaction to the Bitcoin API.
    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    // Return the transaction ID.
    signed_transaction.compute_txid().to_string()
}
