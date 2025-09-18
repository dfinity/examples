use crate::{
    common::{build_transaction_with_fee, select_utxos_greedy, PrimaryOutput},
    ecdsa::mock_sign_with_ecdsa,
    BitcoinContext,
};
use bitcoin::{
    ecdsa::Signature as BitcoinSignature,
    secp256k1::{ecdsa::Signature as SecpSignature, Message},
    sighash::{EcdsaSighashType, SighashCache},
    Address, AddressType, PublicKey, ScriptBuf, Transaction, TxOut, Witness,
};
use ic_cdk::bitcoin_canister::{MillisatoshiPerByte, Satoshi, Utxo};

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
) -> (Transaction, Vec<TxOut>) {
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
        let utxos_to_spend = select_utxos_greedy(own_utxos, amount, fee).unwrap();
        let (transaction, prevouts) = build_transaction_with_fee(
            utxos_to_spend,
            own_address,
            &PrimaryOutput::Address(dst_address.clone(), amount),
            fee,
        )
        .unwrap();

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            ctx,
            own_public_key,
            own_address,
            transaction.clone(),
            &prevouts,
            vec![], // mock derivation path
            mock_sign_with_ecdsa,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;

        if (tx_vsize * fee_per_vbyte) / 1000 == fee {
            return (transaction, prevouts);
        } else {
            fee = (tx_vsize * fee_per_vbyte) / 1000;
        }
    }
}

// Sign a P2WPKH bitcoin transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2WPKH address.
pub async fn sign_transaction<SignFun, Fut>(
    ctx: &BitcoinContext,
    own_public_key: &PublicKey,
    own_address: &Address,
    mut transaction: Transaction,
    prevouts: &[TxOut],
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = SecpSignature>,
{
    assert_eq!(
        own_address.address_type(),
        Some(AddressType::P2wpkh),
        "Only P2WPKH addresses are supported"
    );

    let transaction_clone = transaction.clone();
    let mut sighash_cache = SighashCache::new(&transaction_clone);

    for (index, input) in transaction.input.iter_mut().enumerate() {
        let script_pubkey = &prevouts[index].script_pubkey;
        let value = prevouts[index].value;
        let sighash = sighash_cache
            .p2wpkh_signature_hash(index, script_pubkey, value, EcdsaSighashType::All)
            .unwrap();

        let message = Message::from(sighash);

        let raw_signature = signer(
            ctx.key_name.to_string(),
            derivation_path.clone(),
            message.as_ref().to_vec(),
        )
        .await;

        let signature = BitcoinSignature {
            signature: raw_signature,
            sighash_type: EcdsaSighashType::All,
        };

        input.script_sig = ScriptBuf::new();
        input.witness = Witness::new();
        input.witness.push(signature.to_vec());
        input.witness.push(own_public_key.to_bytes());
    }

    transaction
}
