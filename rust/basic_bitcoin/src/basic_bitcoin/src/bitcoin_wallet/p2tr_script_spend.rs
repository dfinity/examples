use crate::{bitcoin_api, schnorr_api};
use bitcoin::{
    blockdata::witness::Witness,
    consensus::serialize,
    hashes::Hash,
    key::XOnlyPublicKey,
    secp256k1::{schnorr::Signature, PublicKey, Secp256k1},
    sighash::{SighashCache, TapSighashType},
    taproot::{ControlBlock, LeafVersion, TapLeafHash, TaprootBuilder, TaprootSpendInfo},
    Address, AddressType, ScriptBuf, Sequence, Transaction, TxOut, Txid,
};
use ic_cdk::api::management_canister::bitcoin::{
    BitcoinNetwork, MillisatoshiPerByte, Satoshi, Utxo,
};
use ic_cdk::print;
use std::str::FromStr;

/// Returns the P2TR address of this canister at the given derivation path.
pub async fn get_address(
    network: BitcoinNetwork,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> Address {
    let public_key = schnorr_api::schnorr_public_key(key_name, derivation_path).await;
    public_key_to_p2tr_script_spend_address(network, public_key.as_slice())
}

// Converts a public key to a P2TR address. To compute the address, the public
// key is tweaked with the taproot value, which is computed from the public key
// and the Merkelized Abstract Syntax Tree (MAST, essentially a Merkle tree
// containing scripts, in our case just one). Addresses are computed differently
// for different Bitcoin networks.
pub fn public_key_to_p2tr_script_spend_address(
    bitcoin_network: BitcoinNetwork,
    public_key: &[u8],
) -> Address {
    let network = super::common::transform_network(bitcoin_network);
    let taproot_spend_info = p2tr_scipt_spend_info(public_key);
    Address::p2tr_tweaked(taproot_spend_info.output_key(), network)
}

fn p2tr_scipt_spend_info(public_key: &[u8]) -> TaprootSpendInfo {
    let spend_script = p2tr_script(public_key);
    let secp256k1_engine = Secp256k1::new();
    // This is the key used in the *tweaked* key path spending. Currently, this
    // use case is not supported on the IC. But, once the IC supports this use
    // case, the addresses constructed in this way will be able to use same key
    // in both script and *tweaked* key path spending.
    let internal_public_key = XOnlyPublicKey::from(PublicKey::from_slice(&public_key).unwrap());

    TaprootBuilder::new()
        .add_leaf(0, spend_script.clone())
        .expect("adding leaf should work")
        .finalize(&secp256k1_engine, internal_public_key)
        .expect("finalizing taproot builder should work")
}

/// Computes a simple P2TR script that allows the `public_key` and no other keys
/// to be used for spending.
fn p2tr_script(public_key: &[u8]) -> ScriptBuf {
    let x_only_public_key = XOnlyPublicKey::from(PublicKey::from_slice(public_key).unwrap());
    bitcoin::blockdata::script::Builder::new()
        .push_x_only_key(&x_only_public_key)
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSIG)
        .into_script()
}

/// Sends a P2TR script spend transaction to the network that transfers the
/// given amount to the given destination, where the source of the funds is the
/// canister itself at the given derivation path.
pub async fn send(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public key, P2PKH address, and UTXOs.
    let own_public_key =
        schnorr_api::schnorr_public_key(key_name.clone(), derivation_path.clone()).await;
    let taproot_spend_info = p2tr_scipt_spend_info(own_public_key.as_slice());

    let own_address = Address::p2tr_tweaked(
        taproot_spend_info.output_key(),
        super::common::transform_network(network),
    );

    print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = bitcoin_api::get_utxos(network, own_address.to_string())
        .await
        .utxos;

    let dst_address = Address::from_str(&dst_address)
        .unwrap()
        .require_network(super::common::transform_network(network))
        .expect("should be valid address for the network");

    let script = p2tr_script(own_public_key.as_slice());
    let control_block = taproot_spend_info
        .control_block(&(script.clone(), LeafVersion::TapScript))
        .expect("should compute control block");
    // Build the transaction that sends `amount` to the destination address.
    let (transaction, prevouts) = build_p2tr_script_path_spend_tx(
        &own_address,
        &control_block,
        &script,
        &own_utxos,
        &dst_address,
        amount,
        fee_per_byte,
    )
    .await;

    let tx_bytes = serialize(&transaction);
    print(format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = schnorr_sign_script_spend_transaction(
        &own_address,
        transaction,
        prevouts.as_slice(),
        &control_block,
        &script,
        key_name,
        derivation_path,
        schnorr_api::sign_with_schnorr,
    )
    .await;

    let signed_transaction_bytes = serialize(&signed_transaction);
    print(format!(
        "Signed transaction: {}",
        hex::encode(&signed_transaction_bytes)
    ));

    print("Sending transaction...");
    bitcoin_api::send_transaction(network, signed_transaction_bytes).await;
    print("Done");

    signed_transaction.txid()
}

// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
async fn build_p2tr_script_path_spend_tx(
    own_address: &Address,
    control_block: &ControlBlock,
    script: &ScriptBuf,
    own_utxos: &[Utxo],
    dst_address: &Address,
    amount: Satoshi,
    fee_per_byte: MillisatoshiPerByte,
) -> (Transaction, Vec<TxOut>) {
    // We have a chicken-and-egg problem where we need to know the length
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    print("Building transaction...");
    let mut total_fee = 0;
    loop {
        let (transaction, prevouts) = super::common::build_transaction_with_fee(
            own_utxos,
            own_address,
            dst_address,
            amount,
            total_fee,
        )
        .expect("Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = schnorr_sign_script_spend_transaction(
            own_address,
            transaction.clone(),
            &prevouts,
            control_block,
            script,
            String::from(""), // mock key name
            vec![],           // mock derivation path
            super::common::mock_signer,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;

        if (tx_vsize * fee_per_byte) / 1000 == total_fee {
            print(format!("Transaction built with fee {}.", total_fee));
            return (transaction, prevouts);
        } else {
            total_fee = (tx_vsize * fee_per_byte) / 1000;
        }
    }
}

// Sign a P2TR script spend transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2TR script path spend address.
async fn schnorr_sign_script_spend_transaction<SignFun, Fut>(
    own_address: &Address,
    mut transaction: Transaction,
    prevouts: &[TxOut],
    control_block: &ControlBlock,
    script: &ScriptBuf,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Vec<u8>>,
{
    assert_eq!(own_address.address_type(), Some(AddressType::P2tr),);

    for input in transaction.input.iter_mut() {
        input.script_sig = ScriptBuf::default();
        input.witness = Witness::default();
        input.sequence = Sequence::ENABLE_RBF_NO_LOCKTIME;
    }

    let num_inputs = transaction.input.len();

    for i in 0..num_inputs {
        let mut sighasher = SighashCache::new(&mut transaction);

        let leaf_hash = TapLeafHash::from_script(&script, LeafVersion::TapScript);

        let signing_data = sighasher
            .taproot_script_spend_signature_hash(
                i,
                &bitcoin::sighash::Prevouts::All(&prevouts),
                //      &bitcoin::sighash::Prevouts::All(&prevouts[i..i + 1]),
                leaf_hash,
                TapSighashType::Default,
            )
            .expect("Failed to ecnode signing data")
            .as_byte_array()
            .to_vec();

        let raw_signature = signer(
            key_name.clone(),
            derivation_path.clone(),
            signing_data.clone(),
        )
        .await;

        // Update the witness stack.

        let witness = sighasher.witness_mut(i).unwrap();
        witness.clear();
        let signature = bitcoin::taproot::Signature {
            sig: Signature::from_slice(&raw_signature).expect("failed to parse signature"),
            hash_ty: TapSighashType::Default,
        };
        witness.push(signature.to_vec());
        witness.push(&script.to_bytes());
        witness.push(control_block.serialize());
    }

    transaction
}
