// This module implements BRC-20 token transfer functionality.
//
// A BRC-20 transfer requires two steps per the protocol specification:
// 1. Inscribe a transfer: Create an inscription with the transfer JSON at the SENDER's
//    address. This "locks" the specified token amount from the sender's balance.
// 2. Send the inscription: Move the UTXO holding that inscription to the RECIPIENT's
//    address. BRC-20 indexers credit the recipient when they see the inscription move.
//
// This function chains three Bitcoin transactions in a single canister call:
//   Commit TX:  Funds the Taproot commit address (contains the BRC-20 transfer script).
//   Reveal TX:  Spends the commit output, revealing the JSON and outputting the inscription
//               to the canister's own address (i.e. the sender).
//   Send TX:    Spends the inscription UTXO from the canister's address to the recipient.

use crate::{
    brc20::commit_and_reveal,
    common::{get_fee_per_byte, DerivationPath},
    p2tr,
    schnorr::{get_schnorr_public_key, mock_sign_with_schnorr, sign_with_schnorr},
    TransferBrc20Request, BTC_CONTEXT,
};
use bitcoin::{
    absolute::LockTime,
    blockdata::witness::Witness,
    consensus::serialize,

    secp256k1::{PublicKey, Secp256k1},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, XOnlyPublicKey,
};
use ic_cdk::{trap, update};
use ic_cdk_bitcoin_canister::{bitcoin_send_transaction, SendTransactionRequest};
use std::str::FromStr;

/// Transfers BRC-20 tokens from the canister's address to a recipient.
///
/// Three transactions are chained within this call:
/// 1. Commit: Sends funds to a Taproot address committed to the transfer script.
/// 2. Reveal: Reveals the transfer JSON at the canister's own address, locking
///    `amount` tokens from the canister's BRC-20 balance into the inscription.
/// 3. Send: Moves the inscription UTXO to the recipient, crediting their balance.
///
/// BRC-20 indexers recognise the inscription travelling from sender → recipient as
/// a token transfer and update balances accordingly.
///
/// The canister must have enough minted balance before calling this function. Use
/// `mint_brc20` to acquire tokens after deploying with `inscribe_brc20`.
#[update]
pub async fn transfer_brc20(request: TransferBrc20Request) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if request.tick.len() != 4 {
        trap("BRC-20 ticker must be exactly 4 characters");
    }

    if request.amount == 0 {
        trap("Amount must be greater than 0");
    }

    let recipient = Address::from_str(&request.destination_address)
        .unwrap()
        .require_network(ctx.bitcoin_network)
        .unwrap();

    let tick = request.tick.to_uppercase();

    // Derive the canister's P2TR funding address (the "sender" whose balance is debited).
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key_bytes =
        get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key =
        XOnlyPublicKey::from(PublicKey::from_slice(&internal_key_bytes).unwrap());
    let secp = Secp256k1::new();
    let own_address = Address::p2tr(&secp, internal_key, None, ctx.bitcoin_network);

    // BRC-20 transfer JSON: locks `amount` tokens from the sender's balance.
    let brc20_json = format!(
        r#"{{"p":"brc-20","op":"transfer","tick":"{}","amt":"{}"}}"#,
        tick, request.amount
    );

    // --- Step 1 & 2: Commit + Reveal ---
    // The reveal output goes to the canister's own address, placing the transfer
    // inscription at the sender's address as required by the BRC-20 spec.
    // `commit_and_reveal` returns the reveal txid; the inscription is at vout 0.
    let reveal_txid_str = commit_and_reveal(&ctx, &brc20_json, &own_address).await;

    // --- Step 3: Send the inscription UTXO to the recipient ---
    // Spending reveal_txid:0 moves the inscription from sender to recipient,
    // which BRC-20 indexers interpret as the token transfer completing.
    let reveal_txid = Txid::from_str(&reveal_txid_str).unwrap();

    // The reveal output value was set by ordinals::INSCRIPTION_OUTPUT_VALUE minus reveal fee.
    // We query it from the reveal transaction's known constant rather than waiting for
    // confirmation — Bitcoin allows spending unconfirmed outputs (CPFP).
    // Use a conservative estimate: INSCRIPTION_OUTPUT_VALUE (15_000 sat) minus a
    // generous max reveal fee of 2_000 sat leaves at least 13_000 sat for the send step.
    // The iterative fee loop below adjusts the exact send output value.
    const MIN_INSCRIPTION_VALUE: u64 = 13_000;

    let send_prevout = TxOut {
        value: Amount::from_sat(MIN_INSCRIPTION_VALUE),
        script_pubkey: own_address.script_pubkey(),
    };

    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let mut send_fee = 0u64;

    let signed_send_tx = loop {
        let output_value = MIN_INSCRIPTION_VALUE
            .checked_sub(send_fee)
            .unwrap_or_else(|| trap("inscription value insufficient to cover send fee"));

        let send_tx = Transaction {
            version: Version::TWO,
            lock_time: LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: reveal_txid,
                    vout: 0,
                },
                sequence: Sequence::MAX,
                witness: Witness::new(),
                script_sig: ScriptBuf::new(),
            }],
            output: vec![TxOut {
                script_pubkey: recipient.script_pubkey(),
                value: Amount::from_sat(output_value),
            }],
        };

        let mock_signed = p2tr::sign_transaction_key_spend(
            &ctx,
            &own_address,
            send_tx.clone(),
            &[send_prevout.clone()],
            vec![],
            vec![],
            mock_sign_with_schnorr,
        )
        .await;

        let new_fee = (mock_signed.vsize() as u64 * fee_per_byte) / 1000;
        if new_fee == send_fee {
            let signed = p2tr::sign_transaction_key_spend(
                &ctx,
                &own_address,
                send_tx,
                &[send_prevout.clone()],
                internal_key_path.to_vec_u8_path(),
                vec![],
                sign_with_schnorr,
            )
            .await;
            break signed;
        }
        send_fee = new_fee;
    };

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&signed_send_tx),
    })
    .await
    .unwrap();

    signed_send_tx.compute_txid().to_string()
}
