use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    SendRequest, BTC_CONTEXT,
};
use bitcoin::{consensus::serialize, hashes::Hash, Address};
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    },
    trap, update,
};
use std::str::FromStr;

/// Sends bitcoin from this smart contract’s **script-path-enabled Taproot address** using **key path spending**.
///
/// This function constructs and broadcasts a transaction that spends from a Taproot output
/// with a committed script tree (i.e. the address was constructed with both an internal key
/// and a Merkle root), but the spend path taken is **key path spending only** — a single
/// Schnorr signature derived from the internal key.
///
/// This is functionally similar to `send_from_p2tr_key_path_only_address`, but uses a different
/// derivation path to reflect that the address also supports an alternate script path.
#[update]
pub async fn send_from_p2tr_script_path_enabled_address_key_spend(request: SendRequest) -> String {
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
    let internal_key_path = DerivationPath::p2tr(0, 1);
    let script_leaf_key_path = DerivationPath::p2tr(0, 2);

    // Derive the Schnorr public keys used in this Taproot output:
    // - `internal_key` is used as the untweaked base key (for key path spending)
    // - `script_key` is used inside a Taproot leaf script (for script path spending)
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let script_key = get_schnorr_public_key(&ctx, script_leaf_key_path.to_vec_u8_path()).await;

    // Construct the Taproot leaf script: <script_key> OP_CHECKSIG
    // This is a simple script that allows spending via the script_key alone.
    let taproot_spend_info = p2tr::create_taproot_spend_info(&internal_key, &script_key);

    // Construct the Taproot address. The address encodes the tweaked output key and is
    // network-aware (mainnet, testnet, etc.).
    let own_address = Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);

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
        taproot_spend_info
            .merkle_root()
            .unwrap()
            .as_byte_array()
            .to_vec(),
        sign_with_schnorr,
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    signed_transaction.compute_txid().to_string()
}
