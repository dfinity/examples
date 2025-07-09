use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    SendRequest, BTC_CONTEXT,
};
use bitcoin::{consensus::serialize, key::Secp256k1, Address, PublicKey, XOnlyPublicKey};
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    },
    trap, update,
};
use std::str::FromStr;

/// Sends bitcoin from this smart contract’s **key-path-only Taproot address** (P2TR, BIP-86).
///
/// This function constructs and broadcasts a transaction that spends from a Taproot output
/// using **key path spending only** — that is, a single Schnorr signature derived from the
/// internal key with **no script path committed** (the Merkle root is `None`).
#[update]
pub async fn send_from_p2tr_key_path_only_address(request: SendRequest) -> String {
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

    // Derivation path strategy:
    // We assign fixed address indexes for key roles within Taproot:
    // - Index 0: key-path-only Taproot (no script tree committed)
    // - Index 1: internal key for a Taproot output that includes a script tree
    // - Index 2: script leaf key committed to in the Merkle tree
    let internal_key_path = DerivationPath::p2tr(0, 0);

    // Derive the public key used as the internal key (untweaked key path base).
    // This key is used for key path spending only, without any committed script tree.
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;

    // Convert the internal key to an x-only public key, as required by Taproot (BIP-341).
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Create a Taproot address using the internal key only.
    // We pass `None` as the Merkle root, which per BIP-341 means the address commits
    // to an unspendable script path, enabling only key path spending.
    let secp256k1_engine = Secp256k1::new();
    let own_address = Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network);

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

    // Build the transaction
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &own_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Greedy,
        &PrimaryOutput::Address(dst_address, request.amount_in_satoshi),
        fee_per_byte,
    )
    .await;

    // Sign the transaction.
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &own_address,
        transaction,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
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
