// Rune etching uses a commit+reveal pattern required by the ord protocol.
//
// Step 1 — commit_rune: Create a P2TR output whose tapscript contains the rune commitment
//           bytes (rune name encoded as u128 LE, trailing zeros stripped). Wait 6 blocks.
//
// Step 2 — etch_rune:   Spend that commit output via script-path (which places the tapscript
//           in the witness). The etching transaction also contains an OP_RETURN Runestone.
//           The ord indexer finds the commitment bytes in the witness, verifies 6+ confirmations
//           on the commit output, and assigns a Rune ID.
//
// Without a valid committed input the indexer silently ignores the etching (no Rune ID assigned).

use crate::{
    common::{get_fee_per_byte, DerivationPath, PrimaryOutput},
    ordinals::{build_rune_etch_transaction, create_script_path_witness, INSCRIPTION_OUTPUT_VALUE},
    p2tr,
    runes::{build_etching_script, build_rune_commit_script, Etching},
    schnorr::{get_schnorr_public_key, sign_with_schnorr},
    BTC_CONTEXT,
};
use bitcoin::{
    consensus::serialize,
    secp256k1::{PublicKey, Secp256k1},
    taproot::{LeafVersion, TaprootBuilder, TaprootSpendInfo},
    Address, Amount, ScriptBuf, TxOut, Txid, XOnlyPublicKey,
};
use ic_cdk::{trap, update};
use ic_cdk_bitcoin_canister::{
    bitcoin_get_utxos, bitcoin_send_transaction, GetUtxosRequest, SendTransactionRequest,
    UtxosFilterInRequest,
};

fn validate_rune_name(name: &str) {
    if name.is_empty() {
        trap("Rune name cannot be empty");
    }
    if name.len() > 28 {
        trap("Rune name cannot exceed 28 characters");
    }
    if !name.chars().all(|c| c.is_ascii_uppercase()) {
        trap("Rune name must contain only uppercase letters A-Z");
    }
}

/// Derives the commit script, Taproot spend info, commit address, and funding address for a rune.
///
/// Both `commit_rune` and `etch_rune` use the same derivation so the commit address is
/// deterministic and `etch_rune` can locate the commit UTXO without being told the txid.
async fn rune_commit_info(
    name: &str,
) -> (ScriptBuf, TaprootSpendInfo, Address, Address) {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let internal_key_path = DerivationPath::p2tr(0, 0);
    let internal_key_bytes =
        get_schnorr_public_key(&ctx, internal_key_path.to_vec_u8_path()).await;
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key_bytes).unwrap());

    let commit_script = build_rune_commit_script(&internal_key, name)
        .unwrap_or_else(|e| trap(format!("Failed to build commit script: {}", e)));

    let secp = Secp256k1::new();
    let taproot_spend_info = TaprootBuilder::new()
        .add_leaf(0, commit_script.clone())
        .unwrap()
        .finalize(&secp, internal_key)
        .unwrap();

    let commit_address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network);
    let funding_address = Address::p2tr(&secp, internal_key, None, ctx.bitcoin_network);

    (commit_script, taproot_spend_info, commit_address, funding_address)
}

/// Step 1 of rune etching — creates and broadcasts the commit transaction.
///
/// Sends `INSCRIPTION_OUTPUT_VALUE` satoshis to a P2TR address whose tapscript contains the
/// rune commitment bytes. The commit output must have at least 6 block confirmations before
/// calling `etch_rune`.
///
/// Returns the commit transaction ID.
#[update]
pub async fn commit_rune(name: String) -> String {
    validate_rune_name(&name);

    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let internal_key_path = DerivationPath::p2tr(0, 0);

    let (_, _, commit_address, funding_address) = rune_commit_info(&name).await;

    let own_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: funding_address.to_string(),
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
    .utxos;

    let fee_per_byte = get_fee_per_byte(&ctx).await;

    let (commit_tx, prevouts) = p2tr::build_transaction(
        &ctx,
        &funding_address,
        &own_utxos,
        p2tr::SelectUtxosMode::Single,
        &PrimaryOutput::Address(commit_address, INSCRIPTION_OUTPUT_VALUE),
        fee_per_byte,
    )
    .await;

    // funding_address is a key-path-only P2TR (no script), so merkle root is empty.
    let signed_commit = p2tr::sign_transaction_key_spend(
        &ctx,
        &funding_address,
        commit_tx,
        prevouts.as_slice(),
        internal_key_path.to_vec_u8_path(),
        vec![],
        sign_with_schnorr,
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&signed_commit),
    })
    .await
    .unwrap();

    signed_commit.compute_txid().to_string()
}

/// Step 2 of rune etching — spends the commit output and broadcasts the etching transaction.
///
/// Must be called after the commit output has at least 6 block confirmations. The etch
/// transaction spends the commit output via script-path spend so that the rune commitment
/// bytes appear in the witness, satisfying the ord indexer's validation.
///
/// Returns the etching transaction ID.
#[update]
pub async fn etch_rune(name: String) -> String {
    validate_rune_name(&name);

    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let internal_key_path = DerivationPath::p2tr(0, 0);

    let (commit_script, taproot_spend_info, commit_address, _funding_address) =
        rune_commit_info(&name).await;

    // Find the commit UTXO with at least 6 confirmations. The Runes protocol requires
    // the commit output to be at least 6 blocks deep before the etching is recognised by indexers.
    let commit_utxos = bitcoin_get_utxos(&GetUtxosRequest {
        address: commit_address.to_string(),
        network: ctx.network.into(),
        filter: Some(UtxosFilterInRequest::MinConfirmations(6)),
    })
    .await
    .unwrap()
    .utxos;

    if commit_utxos.is_empty() {
        trap("No commit output with 6+ confirmations found. Call commit_rune first and mine 6 blocks before calling etch_rune.");
    }

    let commit_utxo = &commit_utxos[0];

    let commit_txid = Txid::from_raw_hash(
        bitcoin::hashes::Hash::from_slice(commit_utxo.outpoint.txid.as_ref()).unwrap(),
    );
    let commit_vout = commit_utxo.outpoint.vout;
    let commit_value = commit_utxo.value;

    let commit_txout = TxOut {
        value: Amount::from_sat(commit_value),
        script_pubkey: commit_address.script_pubkey(),
    };

    let control_block = taproot_spend_info
        .control_block(&(commit_script.clone(), LeafVersion::TapScript))
        .unwrap();

    let etching = Etching {
        divisibility: 0,
        premine: 1_000_000,
        rune_name: name.clone(),
        symbol: Some('🪙'),
        terms: None,
        turbo: true,
        spacers: 0,
    };

    let runestone_script = build_etching_script(&etching)
        .unwrap_or_else(|e| trap(format!("Failed to build runestone: {}", e)));

    // The rune premine goes to a dedicated address (p2tr index 1) that is separate from the
    // main funding address (p2tr index 0). This makes it possible for transfer_rune to spend
    // only rune-bearing UTXOs without querying an external indexer.
    let rune_key_path = DerivationPath::p2tr(0, 1);
    let rune_key_bytes = get_schnorr_public_key(&ctx, rune_key_path.to_vec_u8_path()).await;
    let rune_key = XOnlyPublicKey::from(PublicKey::from_slice(&rune_key_bytes).unwrap());
    let secp2 = Secp256k1::new();
    let rune_address = Address::p2tr(&secp2, rune_key, None, ctx.bitcoin_network);

    let fee_per_byte = get_fee_per_byte(&ctx).await;

    let mut etch_tx = build_rune_etch_transaction(
        &rune_address,
        &runestone_script,
        &commit_script,
        &control_block,
        &commit_txid,
        commit_vout,
        commit_value,
        fee_per_byte,
    )
    .await;

    create_script_path_witness(
        &ctx,
        &mut etch_tx,
        &commit_txout,
        &commit_script,
        &control_block,
        internal_key_path.to_vec_u8_path(),
    )
    .await;

    bitcoin_send_transaction(&SendTransactionRequest {
        network: ctx.network.into(),
        transaction: serialize(&etch_tx),
    })
    .await
    .unwrap();

    etch_tx.compute_txid().to_string()
}
