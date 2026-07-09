// This module implements Bitcoin Runes token transfer functionality.
// A rune transfer creates a Bitcoin transaction with a Runestone in an OP_RETURN
// output. The Runestone contains an edict that tells the protocol how to allocate
// rune balances from the spent UTXOs to the transaction outputs.

use crate::{
    common::{get_fee_per_byte, DerivationPath},
    p2tr,
    runes::{build_transfer_script, Edict},
    schnorr::{get_schnorr_public_key, mock_sign_with_schnorr, sign_with_schnorr},
    TransferRuneRequest, BTC_CONTEXT,
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
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest, Utxo,
};
use std::str::FromStr;

// Minimum satoshi value attached to the recipient output.
// Bitcoin nodes reject outputs below the dust threshold (~330–546 sat depending on type).
// 1000 sat is a safe round number above all thresholds.
const DUST_AMOUNT: u64 = 1_000;

/// Transfers rune tokens from the canister's P2TR address to a recipient address.
///
/// The Runes protocol tracks token balances per UTXO. This function builds a
/// Bitcoin transaction whose Runestone directs the rune balance from the spent
/// input UTXOs to the specified recipient output.
///
/// Transaction output layout (vout indices matter for the edict and pointer):
///   vout[0] — recipient: receives exactly `amount` rune tokens + DUST_AMOUNT satoshis
///   vout[1] — OP_RETURN: Runestone with the transfer edict and pointer (no bitcoin value)
///   vout[2] — change: receives unallocated rune tokens + remaining satoshis
///
/// The Runestone contains:
///   - edict: send `amount` tokens of the given rune ID to vout[0]
///   - pointer: 2 — unallocated tokens (the remainder) go to vout[2]
///
/// Without the pointer, unallocated runes would default to the first non-OP_RETURN
/// output (vout[0]), giving the recipient the entire balance instead of just `amount`.
///
/// The rune ID (`rune_id_block` : `rune_id_tx`) can be found in the ord explorer
/// at http://127.0.0.1/rune/<NAME> after starting `ord --config-dir . server`.
#[update]
pub async fn transfer_rune(request: TransferRuneRequest) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if request.amount == 0 {
        trap("Amount must be greater than 0");
    }

    // Parse and validate the destination address for the current network.
    let destination_address = Address::from_str(&request.destination_address)
        .unwrap_or_else(|e| trap(format!("Invalid destination address: {}", e)))
        .require_network(ctx.bitcoin_network)
        .unwrap_or_else(|e| trap(format!("Address is for wrong network: {}", e)));

    // Rune balances are held at the dedicated rune address (p2tr index 1), separate from the
    // main funding address (p2tr index 0). All UTXOs there are rune-bearing outputs created by
    // etch_rune and previous transfer change outputs — so spending all of them is correct.
    let rune_key_path = DerivationPath::p2tr(0, 1);
    let rune_key = get_schnorr_public_key(&ctx, rune_key_path.to_vec_u8_path()).await;
    let rune_key = XOnlyPublicKey::from(PublicKey::from_slice(&rune_key).unwrap());

    let secp256k1_engine = Secp256k1::new();
    let rune_address = Address::p2tr(&secp256k1_engine, rune_key, None, ctx.bitcoin_network);

    // Fetch all UTXOs at the rune address — every one of them holds rune balance.
    let rune_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: rune_address.to_string(),
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    if rune_utxos.is_empty() {
        trap("No rune UTXOs found. Etch a rune first.");
    }

    // Build the Runestone:
    //   - edict: send `amount` tokens to vout[0] (recipient)
    //   - pointer: 2 — unallocated tokens (the remainder) go to vout[2] (change)
    //
    // Without the pointer, unallocated runes default to the first non-OP_RETURN
    // output (vout[0]), which would give the recipient the entire rune balance
    // instead of just the requested amount.
    let runestone_script = build_transfer_script(
        &[Edict {
            rune_id_block: request.rune_id_block,
            rune_id_tx: request.rune_id_tx,
            amount: request.amount,
            output: 0, // vout[0] is the recipient
        }],
        Some(2), // pointer: unallocated runes go to vout[2] (change)
    )
    .unwrap_or_else(|e| trap(format!("Failed to build runestone: {}", e)));

    let utxos_to_spend: Vec<&_> = rune_utxos.iter().collect();

    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let mut total_fee = 0u64;
    let (transaction, prevouts) = loop {
        let (tx, prevouts) = build_rune_transfer_tx(
            &utxos_to_spend,
            &rune_address,
            &destination_address,
            &runestone_script,
            total_fee,
        );

        let signed_tx = p2tr::sign_transaction_key_spend(
            &ctx,
            &rune_address,
            tx.clone(),
            &prevouts,
            vec![],
            vec![],
            mock_sign_with_schnorr,
        )
        .await;

        let new_fee = (signed_tx.vsize() as u64 * fee_per_byte) / 1000;
        if new_fee == total_fee {
            break (tx, prevouts);
        }
        total_fee = new_fee;
    };

    // Sign with the real Schnorr key and broadcast.
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &rune_address,
        transaction,
        prevouts.as_slice(),
        rune_key_path.to_vec_u8_path(),
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

/// Constructs the unsigned transaction for a rune transfer.
///
/// Output order is significant because the Runestone's edict and pointer reference
/// outputs by their vout index:
///   vout[0] — recipient (edict `output: 0`)
///   vout[1] — OP_RETURN / Runestone
///   vout[2] — change (Runestone `pointer: 2`)
///
/// vout[2] must always exist: if it were absent the pointer would reference a
/// non-existent output, which the ord protocol treats as a cenotaph and burns
/// all rune tokens in the transaction. We trap early if the fee is so large
/// that no change satoshis remain.
fn build_rune_transfer_tx(
    utxos_to_spend: &[&Utxo],
    own_address: &Address,
    destination_address: &Address,
    runestone_script: &ScriptBuf,
    fee: u64,
) -> (Transaction, Vec<TxOut>) {
    let inputs: Vec<TxIn> = utxos_to_spend
        .iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_raw_hash(Hash::from_slice(utxo.outpoint.txid.as_ref()).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: Sequence::MAX,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        })
        .collect();

    let prevouts: Vec<TxOut> = utxos_to_spend
        .iter()
        .map(|utxo| TxOut {
            value: Amount::from_sat(utxo.value),
            script_pubkey: own_address.script_pubkey(),
        })
        .collect();

    let total_in: u64 = utxos_to_spend.iter().map(|u| u.value).sum();
    let change = total_in
        .checked_sub(DUST_AMOUNT + fee)
        .unwrap_or_else(|| trap("fee exceeds inputs"));

    // vout[2] must always be present for the Runestone pointer to be valid.
    // In practice the rune UTXOs always have thousands of satoshis, so this
    // threshold is only hit if something is severely wrong.
    if change < DUST_AMOUNT {
        trap("Insufficient satoshis for change output; fee too high");
    }

    // vout[0]: recipient — edict `output: 0`, receives `amount` rune tokens
    // vout[1]: OP_RETURN — Runestone (edict + pointer), carries no bitcoin value
    // vout[2]: change — Runestone `pointer: 2`, receives unallocated rune tokens
    let outputs = vec![
        TxOut {
            script_pubkey: destination_address.script_pubkey(),
            value: Amount::from_sat(DUST_AMOUNT),
        },
        TxOut {
            script_pubkey: runestone_script.clone(),
            value: Amount::from_sat(0),
        },
        TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: Amount::from_sat(change),
        },
    ];

    (
        Transaction {
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
            version: Version::TWO,
        },
        prevouts,
    )
}
