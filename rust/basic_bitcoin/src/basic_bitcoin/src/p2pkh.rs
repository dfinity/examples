use crate::{
    common::{build_transaction_with_fee, sec1_to_der},
    ecdsa::mock_sign_with_ecdsa,
    BitcoinContext,
};
use bitcoin::{
    hashes::Hash,
    script::{Builder, PushBytesBuf},
    sighash::{EcdsaSighashType, SighashCache},
    Address, AddressType, PublicKey, Transaction,
};
use ic_cdk::bitcoin_canister::{MillisatoshiPerByte, Satoshi, Utxo};
use std::convert::TryFrom;

const ECDSA_SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
pub async fn build_transaction(
    ctx: &BitcoinContext,
    own_public_key: &PublicKey,
    own_address: &Address,
    own_utxos: &[Utxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_vbyte: MillisatoshiPerByte,
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
        let (transaction, _) =
            build_transaction_with_fee(own_utxos, own_address, dst_address, amount, fee).unwrap();

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            ctx,
            own_public_key,
            own_address,
            transaction.clone(),
            vec![], // mock derivation path
            mock_sign_with_ecdsa,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;

        if (tx_vsize * fee_per_vbyte) / 1000 == fee {
            return transaction;
        } else {
            fee = (tx_vsize * fee_per_vbyte) / 1000;
        }
    }
}

// Sign a bitcoin transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2PKH address.
pub async fn sign_transaction<SignFun, Fut>(
    ctx: &BitcoinContext,
    own_public_key: &PublicKey,
    own_address: &Address,
    mut transaction: Transaction,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    // Verify that our own address is P2PKH.
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2pkh),
        "This example supports signing p2pkh addresses only."
    );

    let txclone = transaction.clone();
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash = SighashCache::new(&txclone)
            .legacy_signature_hash(
                index,
                &own_address.script_pubkey(),
                ECDSA_SIG_HASH_TYPE.to_u32(),
            )
            .unwrap();

        let signature = signer(
            ctx.key_name.to_string(),
            derivation_path.clone(),
            sighash.as_byte_array().to_vec(),
        )
        .await;

        // Convert signature to DER.
        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype: Vec<u8> = der_signature;
        sig_with_hashtype.push(ECDSA_SIG_HASH_TYPE.to_u32() as u8);

        let sig_with_hashtype_push_bytes = PushBytesBuf::try_from(sig_with_hashtype).unwrap();
        let own_public_key_push_bytes = PushBytesBuf::try_from(own_public_key.to_bytes()).unwrap();
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype_push_bytes)
            .push_slice(own_public_key_push_bytes)
            .into_script();
        input.witness.clear();
    }

    transaction
}
