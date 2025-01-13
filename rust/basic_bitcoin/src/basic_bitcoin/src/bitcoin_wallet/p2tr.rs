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

/// Returns the P2TR address of this canister at the given derivation path. This
/// address uses two public keys:
/// 1) an internal key,
/// 2) a key that can be used to spend from a script.
///
/// The keys are derived by appending additional information to the provided
/// `derivation_path`.
pub async fn get_address(
    network: BitcoinNetwork,
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
) -> Address {
    let (internal_key, script_path_key) = get_public_keys(key_name, derivation_path).await;
    public_keys_to_p2tr_script_spend_address(
        network,
        internal_key.as_slice(),
        script_path_key.as_slice(),
    )
}

// Converts a public key to a P2TR address. To compute the address, the public
// key is tweaked with the taproot value, which is computed from the public key
// and the Merkelized Abstract Syntax Tree (MAST, essentially a Merkle tree
// containing scripts, in our case just one). Addresses are computed differently
// for different Bitcoin networks.
pub fn public_keys_to_p2tr_script_spend_address(
    bitcoin_network: BitcoinNetwork,
    internal_key: &[u8],
    script_key: &[u8],
) -> Address {
    let network = super::common::transform_network(bitcoin_network);
    let taproot_spend_info = p2tr_script_spend_info(internal_key, script_key);
    Address::p2tr_tweaked(taproot_spend_info.output_key(), network)
}

fn p2tr_script_spend_info(internal_key_bytes: &[u8], script_key_bytes: &[u8]) -> TaprootSpendInfo {
    // Script used in script path spending.
    let spend_script = p2tr_script(script_key_bytes);
    let secp256k1_engine = Secp256k1::new();
    // Key used in the key path spending.
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(&internal_key_bytes).unwrap());

    // Taproot with an internal key and a single script.
    TaprootBuilder::new()
        .add_leaf(0, spend_script.clone())
        .expect("adding leaf should work")
        .finalize(&secp256k1_engine, internal_key)
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
pub async fn send_script_path(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public keys and UTXOs, and compute the P2TR address.
    let (internal_key, script_key) =
        get_public_keys(key_name.clone(), derivation_path.clone()).await;
    let taproot_spend_info = p2tr_script_spend_info(internal_key.as_slice(), script_key.as_slice());

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

    let script = p2tr_script(script_key.as_slice());
    let control_block = taproot_spend_info
        .control_block(&(script.clone(), LeafVersion::TapScript))
        .expect("should compute control block");
    // Build the transaction that sends `amount` to the destination address.
    let (transaction, prevouts) =
        build_p2tr_tx(&own_address, &own_utxos, &dst_address, amount, fee_per_byte).await;

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
        extend_derivation_path_2(&derivation_path).1,
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

/// Sends a P2TR key spend transaction to the network that transfers the
/// given amount to the given destination, where the source of the funds is the
/// canister itself at the given derivation path.
pub async fn send_key_path(
    network: BitcoinNetwork,
    derivation_path: Vec<Vec<u8>>,
    key_name: String,
    dst_address: String,
    amount: Satoshi,
) -> Txid {
    let fee_per_byte = super::common::get_fee_per_byte(network).await;

    // Fetch our public key, P2PKH address, and UTXOs.
    let (internal_key, script_key) =
        get_public_keys(key_name.clone(), derivation_path.clone()).await;
    let taproot_spend_info = p2tr_script_spend_info(internal_key.as_slice(), script_key.as_slice());

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

    // Build the transaction that sends `amount` to the destination address.
    let (transaction, prevouts) =
        build_p2tr_tx(&own_address, &own_utxos, &dst_address, amount, fee_per_byte).await;

    let tx_bytes = serialize(&transaction);
    print(format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign the transaction.
    let signed_transaction = schnorr_sign_key_spend_transaction(
        &own_address,
        transaction,
        prevouts.as_slice(),
        key_name,
        extend_derivation_path_2(&derivation_path).0,
        taproot_spend_info
            .merkle_root()
            .unwrap()
            .as_byte_array()
            .to_vec(),
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

// Builds a P2TR transaction to send the given `amount` of satoshis to the
// destination address.
pub(crate) async fn build_p2tr_tx(
    own_address: &Address,
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
        // of the signed transaction, so we use a mock signer here for
        // efficiency.
        //
        // Note: it doesn't matter which particular spending path to use, key or
        // script path, since the difference is only how the signature is
        // computed, which is a dummy signing function in our case.
        let signed_transaction = schnorr_sign_key_spend_transaction(
            own_address,
            transaction.clone(),
            &prevouts,
            String::from(""), // mock key name
            vec![],           // mock derivation path
            vec![],
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
// 2. `own_address` is a P2TR address that includes a script.
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
    SignFun: Fn(String, Vec<Vec<u8>>, Option<Vec<u8>>, Vec<u8>) -> Fut,
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
                leaf_hash,
                TapSighashType::Default,
            )
            .expect("Failed to encode signing data")
            .as_byte_array()
            .to_vec();

        let raw_signature = signer(
            key_name.clone(),
            derivation_path.clone(),
            None,
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

// Sign a P2TR key spend transaction.
//
// IMPORTANT: This method is for demonstration purposes only and it only
// supports signing transactions if:
//
// 1. All the inputs are referencing outpoints that are owned by `own_address`.
// 2. `own_address` is a P2TR address.
pub async fn schnorr_sign_key_spend_transaction<SignFun, Fut>(
    own_address: &Address,
    mut transaction: Transaction,
    prevouts: &[TxOut],
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    merkle_root_hash: Vec<u8>,
    signer: SignFun,
) -> Transaction
where
    SignFun: Fn(String, Vec<Vec<u8>>, Option<Vec<u8>>, Vec<u8>) -> Fut,
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

        let signing_data = sighasher
            .taproot_key_spend_signature_hash(
                i,
                &bitcoin::sighash::Prevouts::All(&prevouts),
                TapSighashType::Default,
            )
            .expect("Failed to encode signing data")
            .as_byte_array()
            .to_vec();

        let raw_signature = signer(
            key_name.clone(),
            derivation_path.clone(),
            Some(merkle_root_hash.clone()),
            signing_data.clone(),
        )
        .await;

        // Update the witness stack.
        let witness = sighasher.witness_mut(i).unwrap();
        let signature = bitcoin::taproot::Signature {
            sig: Signature::from_slice(&raw_signature).expect("failed to parse signature"),
            hash_ty: TapSighashType::Default,
        };
        witness.push(&signature.to_vec());
    }

    transaction
}

/// Derives two public keys by appending different  additional information to
/// the `derivation_path`.
async fn get_public_keys(key_name: String, derivation_path: Vec<Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let (dpkp, dpsp) = extend_derivation_path_2(&derivation_path);
    let internal_key = schnorr_api::schnorr_public_key(key_name.clone(), dpkp).await;
    let script_path_key = schnorr_api::schnorr_public_key(key_name, dpsp).await;
    (internal_key, script_path_key)
}

/// Appends two constant strings to the derivation path to produce two different keys.
fn extend_derivation_path_2(derivation_path: &[Vec<u8>]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let derivation_path_key_path = derivation_path
        .iter()
        .cloned()
        .chain(vec![b"key_path".to_vec()])
        .collect();
    let derivation_path_script_path = derivation_path
        .iter()
        .cloned()
        .chain(vec![b"script_path".to_vec()])
        .collect();

    (derivation_path_key_path, derivation_path_script_path)
}
