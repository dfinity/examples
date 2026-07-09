// Transferring an ordinal inscription means transferring the specific satoshis that
// carry it. The Ordinals protocol assigns inscriptions to the first satoshi of the
// reveal output. To move the inscription, that satoshi must flow to the first output
// (vout 0) of the spending transaction — which is guaranteed by making the inscription
// UTXO the first (and only) input.
//
// The inscription UTXO sits at the canister's main funding address (p2tr(0,0)) with
// a value of INSCRIPTION_OUTPUT_VALUE satoshis (~15,000 sat). The user identifies
// it by the reveal transaction ID returned by inscribe_ordinal.

use crate::{
    common::{get_fee_per_byte, DerivationPath},
    p2tr,
    schnorr::{get_schnorr_public_key, mock_sign_with_schnorr, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    absolute::LockTime,
    blockdata::witness::Witness,
    consensus::serialize,
    hashes::Hash,
    secp256k1::{PublicKey, Secp256k1},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, XOnlyPublicKey,
};
use ic_cdk::{trap, update};
use ic_cdk_bitcoin_canister::{
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
};
use std::str::FromStr;

/// Transfers an Ordinal inscription to a destination address.
///
/// An inscription is bound to the first satoshi of the reveal output. To preserve this
/// binding through the transfer, the inscription UTXO must be the first input of the
/// spending transaction — that satoshi then flows to the first output (vout 0), which
/// is the recipient. Any remaining satoshis (after the fee) go to the same output.
///
/// `reveal_txid` is the transaction ID returned by `inscribe_ordinal`.
///
/// Returns the transfer transaction ID.
#[update]
pub async fn transfer_ordinal(reveal_txid: String, destination_address: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Parse and validate the destination address.
    let destination_address = Address::from_str(&destination_address)
        .unwrap()
        .require_network(ctx.bitcoin_network)
        .unwrap();

    // Parse the reveal txid provided by the caller.
    let reveal_txid = Txid::from_str(&reveal_txid)
        .unwrap_or_else(|_| trap("Invalid reveal txid"));

    // The inscription output lives at the funding address (p2tr index 0) —
    // the same address the reveal transaction sent its output to.
    let key_path = DerivationPath::p2tr(0, 0);
    let pub_key = get_schnorr_public_key(&ctx, key_path.to_vec_u8_path()).await;
    let pub_key = XOnlyPublicKey::from(PublicKey::from_slice(&pub_key).unwrap());
    let secp = Secp256k1::new();
    let funding_address = Address::p2tr(&secp, pub_key, None, ctx.bitcoin_network);

    // Find the inscription UTXO: the output at vout 0 of the reveal transaction.
    let utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: funding_address.to_string(),
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    let inscription_utxo = utxos
        .iter()
        .find(|u| {
            u.outpoint.txid.as_ref()
                == reveal_txid.as_raw_hash().as_byte_array().as_slice()
                && u.outpoint.vout == 0
        })
        .unwrap_or_else(|| {
            trap("Inscription UTXO not found. Check the reveal txid and ensure it is confirmed.")
        });

    let inscription_value = inscription_utxo.value;

    // Build the transfer transaction using an iterative fee loop.
    // The inscription UTXO is the sole input so its value must cover the fee.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let mut total_fee = 0u64;
    let (transaction, prevouts) = loop {
        if total_fee >= inscription_value {
            trap("Fee exceeds inscription output value");
        }

        let tx = build_ordinal_transfer_tx(
            &reveal_txid,
            inscription_value,
            &funding_address,
            &destination_address,
            total_fee,
        );

        // Sign with a dummy key to get the correct virtual size for fee estimation.
        let signed_tx = p2tr::sign_transaction_key_spend(
            &ctx,
            &funding_address,
            tx.clone(),
            &[TxOut {
                value: Amount::from_sat(inscription_value),
                script_pubkey: funding_address.script_pubkey(),
            }],
            vec![],
            vec![],
            mock_sign_with_schnorr,
        )
        .await;

        let new_fee = (signed_tx.vsize() as u64 * fee_per_byte) / 1000;
        if new_fee == total_fee {
            let prevouts = vec![TxOut {
                value: Amount::from_sat(inscription_value),
                script_pubkey: funding_address.script_pubkey(),
            }];
            break (tx, prevouts);
        }
        total_fee = new_fee;
    };

    // Sign with the real key and broadcast.
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &funding_address,
        transaction,
        prevouts.as_slice(),
        key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    signed_transaction.compute_txid().to_string()
}

/// Builds the unsigned ordinal transfer transaction.
///
/// The inscription UTXO is the sole input. The inscription satoshi (the first sat of
/// the input) flows to vout 0 because Bitcoin assigns satoshis to outputs in order.
/// Any value left after the fee follows the same satoshi ordering, so vout 0 receives
/// everything — the inscription and the remaining satoshis.
fn build_ordinal_transfer_tx(
    reveal_txid: &Txid,
    inscription_value: u64,
    _own_address: &Address,
    destination_address: &Address,
    fee: u64,
) -> Transaction {
    // INSCRIPTION_OUTPUT_VALUE (15,000 sat) is safely above dust even after fees;
    // the caller already traps if fee >= inscription_value.
    let output_value = inscription_value.saturating_sub(fee);

    Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: reveal_txid.to_owned(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(output_value),
            script_pubkey: destination_address.script_pubkey(),
        }],
    }
}
