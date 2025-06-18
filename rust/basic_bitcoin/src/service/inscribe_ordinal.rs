use crate::{
    common::{get_fee_per_byte, DerivationPath},
    p2tr::{self},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    absolute::LockTime,
    consensus::serialize,
    hashes::Hash,
    opcodes::{all::*, OP_FALSE},
    script::{Builder, PushBytesBuf},
    secp256k1::{PublicKey, Secp256k1},
    sighash::{Prevouts, SighashCache},
    taproot::{ControlBlock, LeafVersion, TaprootBuilder},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, TapLeafHash, TapSighashType, Transaction, TxIn,
    TxOut, Txid, Witness, XOnlyPublicKey,
};
use hex::encode;
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, MillisatoshiPerByte,
        SendTransactionRequest,
    },
    trap, update,
};

const SIG64_PLACEHOLDER: [u8; 64] = [0u8; 64];
const INSCRIPTION_VALUE_SAT: u64 = 15_000;

/// Constructs and broadcasts a transaction that creates an Ordinal inscription
/// by sending 10,000 sats to a Taproot output that commits to a reveal script.
#[update]
pub async fn inscribe_ordinal(text: String) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    if text.is_empty() {
        trap("Inscription text cannot be empty");
    }

    // Derive internal key used to construct Taproot address (script commitment)
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key).unwrap());

    // Turn the
    let mut text_bytes = PushBytesBuf::new();
    text_bytes.extend_from_slice(text.as_bytes()).unwrap();

    // Build manual inscription script
    let reveal_script = Builder::new()
        .push_slice(internal_key.serialize()) // 32-byte x-only key
        .push_opcode(OP_CHECKSIG)
        .push_opcode(OP_FALSE)
        .push_opcode(OP_IF)
        .push_slice(b"ord")
        .push_int(1)
        .push_slice(b"text/plain")
        .push_int(0)
        .push_slice(&text_bytes)
        .push_opcode(OP_ENDIF)
        .into_script();

    ic_cdk::println!("reveal_script: {}", reveal_script.to_asm_string());

    // Derive Taproot output key (internal_key + tweak(leaf))
    let secp256k1_engine = Secp256k1::new();
    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, reveal_script.clone())
        .unwrap()
        .finalize(&secp256k1_engine, internal_key)
        .unwrap();

    ic_cdk::println!("Reveal script: {:?}", reveal_script.to_bytes());

    // Construct Taproot output address committing to inscription script
    let inscription_address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);

    // Derive key-path-only address to fund the inscription
    let funding_key = get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let funding_key = XOnlyPublicKey::from(PublicKey::from_slice(&funding_key).unwrap());
    let funding_address = Address::p2tr(&secp256k1_engine, funding_key, None, ctx.bitcoin_network);

    // Query UTXOs from key-path-only address
    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: funding_address.to_string(),
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    // Build commit transaction
    let fee_per_byte = get_fee_per_byte(&ctx).await;
    let (transaction, prevouts) = p2tr::build_transaction(
        &ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single,
        &inscription_address,
        INSCRIPTION_VALUE_SAT,
        fee_per_byte,
    )
    .await;

    // Sign using key-path spend
    let signed_transaction = p2tr::sign_transaction_key_spend(
        &ctx,
        &funding_address,
        transaction,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&signed_transaction),
    })
    .await
    .unwrap();

    let commit_txid = signed_transaction.compute_txid();
    let commit_wtxid = signed_transaction.compute_wtxid();

    ic_cdk::println!("commit txid: {}", commit_txid);
    ic_cdk::println!("commit wtxid: {}", commit_wtxid);
    ic_cdk::println!("commit hex: {}", encode(serialize(&signed_transaction)));

    // --- Begin Reveal Transaction ---

    // Add control block to witness
    let control_block = taproot_spend_info
        .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
        .unwrap();

    let mut reveal_tx = build_reveal_transaction(
        &funding_address,
        &reveal_script,
        &control_block,
        &commit_txid,
        fee_per_byte,
    )
    .await;

    // --- taproot script-path sighash for input 0 ------------------------
    let mut sighasher = SighashCache::new(&mut reveal_tx);
    let prevout = signed_transaction.output[0].clone(); // commit output
    let leaf_hash = TapLeafHash::from_script(&reveal_script, LeafVersion::TapScript);

    let signing_data = sighasher
        .taproot_script_spend_signature_hash(
            0,
            &Prevouts::All(&[prevout]),
            leaf_hash,
            TapSighashType::Default,
        )
        .unwrap()
        .as_byte_array()
        .to_vec();

    ic_cdk::println!("signing_data: {}", hex::encode(signing_data.clone()));

    let raw_signature = sign_with_schnorr(
        ctx.key_name.to_string(),           // << your key name in ctx
        internal_key_path.to_vec_u8_path(), // same derivation used above
        None,                               // no merkle root
        signing_data,
    )
    .await;

    let taproot_script_signature = bitcoin::taproot::Signature {
        signature: bitcoin::secp256k1::schnorr::Signature::from_slice(&raw_signature).unwrap(),
        sighash_type: TapSighashType::Default,
    };

    ic_cdk::println!("sig: {}", hex::encode(&raw_signature));

    let witness = sighasher.witness_mut(0).unwrap();
    witness.clear();
    witness.push(taproot_script_signature.to_vec());
    witness.push(reveal_script.to_bytes());
    witness.push(control_block.serialize());

    ic_cdk::println!("Reveal hex: {}", encode(serialize(&reveal_tx)));

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network,
        transaction: serialize(&reveal_tx),
    })
    .await
    .unwrap();

    reveal_tx.compute_txid().to_string()
}

// Builds a P2TR transaction to send the given `amount` of satoshis to the
// destination address.
pub(crate) async fn build_reveal_transaction(
    dst_address: &Address,
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_txid: &Txid,
    fee_per_byte: MillisatoshiPerByte,
) -> Transaction {
    // We have a chicken-and-egg problem where we need to know the length
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    let mut fee = 0;
    loop {
        let transaction = build_reveal_transaction_with_fee(
            reveal_script,
            control_block,
            commit_txid,
            dst_address,
            fee,
        )
        .unwrap();

        let tx_vsize = transaction.vsize() as u64;
        if (tx_vsize * fee_per_byte) / 1000 == fee {
            return transaction;
        } else {
            fee = (tx_vsize * fee_per_byte) / 1000;
        }
    }
}

fn build_reveal_transaction_with_fee(
    reveal_script: &ScriptBuf,
    control_block: &ControlBlock,
    commit_txid: &Txid,
    dst_address: &Address,
    fee: u64,
) -> Result<Transaction, String> {
    let input = TxIn {
        previous_output: OutPoint {
            txid: commit_txid.to_owned(),
            vout: 0,
        },
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let output = TxOut {
        value: Amount::from_sat(INSCRIPTION_VALUE_SAT - fee),
        script_pubkey: dst_address.script_pubkey(),
    };

    let mut transaction = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![input],
        output: vec![output.clone()],
    };

    transaction.input[0].witness.push(SIG64_PLACEHOLDER);
    transaction.input[0].witness.push(reveal_script.to_bytes());
    transaction.input[0].witness.push(control_block.serialize());

    Ok(transaction)
}
