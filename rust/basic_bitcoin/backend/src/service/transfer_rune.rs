// This module implements Bitcoin Runes token transfer functionality.
// A rune transfer creates a Bitcoin transaction with a Runestone in an OP_RETURN
// output. The Runestone contains an edict that tells the protocol how to allocate
// rune balances from the spent UTXOs to the transaction outputs.

use crate::{
    common::{get_fee_per_byte, select_utxos_greedy, DerivationPath},
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
/// Transaction output layout (vout indices matter for the edict):
///   vout[0] — recipient: receives the rune tokens + DUST_AMOUNT satoshis
///   vout[1] — OP_RETURN: Runestone with the transfer edict (no bitcoin value)
///   vout[2] — change: receives unallocated rune tokens + remaining satoshis
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
        .unwrap()
        .require_network(ctx.bitcoin_network)
        .unwrap();

    // Derive the P2TR key used in etch_rune — same path means same address and rune balance.
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    let secp256k1_engine = Secp256k1::new();
    let own_address = Address::p2tr(&secp256k1_engine, internal_key, None, ctx.bitcoin_network);

    // Fetch UTXOs holding the rune balance at the canister's P2TR address.
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: own_address.to_string(),
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Build the Runestone: a single edict sends `amount` tokens of `rune_id` to vout[0].
    // Unallocated tokens (remainder after the edict) flow to the last non-OP_RETURN output
    // (vout[2], the change output) by default.
    let runestone_script = build_transfer_script(&[Edict {
        rune_id_block: request.rune_id_block,
        rune_id_tx: request.rune_id_tx,
        amount: request.amount,
        output: 0, // vout[0] is the recipient
    }])
    .unwrap_or_else(|e| trap(format!("Failed to build runestone: {}", e)));

    // Iterative fee estimation: mock-sign to measure vsize, adjust fee, repeat until stable.
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let mut total_fee = 0u64;
    let (transaction, prevouts) = loop {
        let utxos_to_spend =
            select_utxos_greedy(&own_utxos, DUST_AMOUNT, total_fee).unwrap_or_else(|e| trap(e));

        let (tx, prevouts) = build_rune_transfer_tx(
            &utxos_to_spend,
            &own_address,
            &destination_address,
            &runestone_script,
            total_fee,
        );

        let signed_tx = p2tr::sign_transaction_key_spend(
            &ctx,
            &own_address,
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
        &own_address,
        transaction,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
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
/// Output order is significant: the edict in the Runestone references outputs by
/// their vout index, so recipient must be at vout[0] for `output: 0` to be correct.
fn build_rune_transfer_tx(
    utxos_to_spend: &[&Utxo],
    own_address: &Address,
    destination_address: &Address,
    runestone_script: &ScriptBuf,
    fee: u64,
) -> (Transaction, Vec<TxOut>) {
    const DUST_THRESHOLD: u64 = 1_000;

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

    // vout[0]: recipient — edict output 0, receives rune tokens + dust satoshis
    // vout[1]: OP_RETURN — Runestone data, carries no bitcoin value
    // vout[2]: change — receives unallocated rune tokens and remaining satoshis
    let mut outputs = vec![
        TxOut {
            script_pubkey: destination_address.script_pubkey(),
            value: Amount::from_sat(DUST_AMOUNT),
        },
        TxOut {
            script_pubkey: runestone_script.clone(),
            value: Amount::from_sat(0),
        },
    ];

    if change >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: own_address.script_pubkey(),
            value: Amount::from_sat(change),
        });
    }

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
