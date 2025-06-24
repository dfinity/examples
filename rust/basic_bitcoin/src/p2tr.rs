use crate::{
    common::{build_transaction_with_fee, select_one_utxo, select_utxos_greedy, PrimaryOutput},
    schnorr::mock_sign_with_schnorr,
    BitcoinContext,
};
use bitcoin::{
    blockdata::witness::Witness,
    hashes::Hash,
    key::XOnlyPublicKey,
    secp256k1::{schnorr::Signature, PublicKey, Secp256k1},
    sighash::{SighashCache, TapSighashType},
    taproot::{ControlBlock, LeafVersion, TapLeafHash, TaprootBuilder, TaprootSpendInfo},
    Address, AddressType, ScriptBuf, Sequence, Transaction, TxOut,
};
use ic_cdk::bitcoin_canister::{MillisatoshiPerByte, Utxo};

/// Constructs the full Taproot spend info for a script-path-enabled Taproot output.
///
/// This function:
/// - Converts the given internal and script leaf public keys into x-only format (as required by BIP-341)
/// - Constructs a script leaf of the form `<script_leaf_key> OP_CHECKSIG`
/// - Commits the script into a Taproot Merkle tree (with a single leaf)
/// - Applies the BIP-341 tweak to the internal key to compute the output key
///
/// The resulting `TaprootSpendInfo` contains the tweaked output key and metadata
/// (including the control block) required for script path spending.
pub fn create_taproot_spend_info(
    internal_key_bytes: &[u8],
    script_key_bytes: &[u8],
) -> TaprootSpendInfo {
    // Convert the internal key to x-only format (required for Taproot tweaking).
    let internal_key = XOnlyPublicKey::from(PublicKey::from_slice(internal_key_bytes).unwrap());

    // Build the script leaf committed to in the Taproot tree.
    // This must exactly match what will be used for script path spending.
    let spend_script = create_spend_script(script_key_bytes);

    // Construct the Taproot output:
    // - A TaprootBuilder is used to create a Merkle tree with one leaf (our spend_script).
    // - The tree is finalized using the internal key, producing a tweaked output key.
    let secp256k1_engine = Secp256k1::new();
    TaprootBuilder::new()
        .add_leaf(0, spend_script.clone())
        .expect("adding leaf should work")
        .finalize(&secp256k1_engine, internal_key)
        .expect("finalizing taproot builder should work")
}

/// Constructs a Taproot leaf script of the form `<script_leaf_key> OP_CHECKSIG`.
///
/// This script is used in Taproot script path spending. It allows spending
/// with a single Schnorr signature corresponding to the committed script leaf key.
///
/// The key must match the one committed in the Taproot output's Merkle tree.
pub fn create_spend_script(script_key_bytes: &[u8]) -> ScriptBuf {
    let script_key = XOnlyPublicKey::from(PublicKey::from_slice(script_key_bytes).unwrap());

    bitcoin::blockdata::script::Builder::new()
        .push_x_only_key(&script_key)
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSIG)
        .into_script()
}

pub enum SelectUtxosMode {
    Greedy,
    Single,
}

// Builds a P2TR transaction to send the given `amount` of satoshis to the
// destination address.
pub(crate) async fn build_transaction(
    ctx: &BitcoinContext,
    own_address: &Address,
    own_utxos: &[Utxo],
    utxos_mode: SelectUtxosMode,
    primary_output: &PrimaryOutput,
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
    let amount = match primary_output {
        PrimaryOutput::Address(_, amount) => *amount,
        PrimaryOutput::OpReturn(_) => 0,
    };
    let mut total_fee = 0;
    loop {
        let utxos_to_spend = match utxos_mode {
            SelectUtxosMode::Greedy => select_utxos_greedy(own_utxos, amount, total_fee),
            SelectUtxosMode::Single => select_one_utxo(own_utxos, amount, total_fee),
        }
        .unwrap();

        let (transaction, prevouts) =
            build_transaction_with_fee(utxos_to_spend, own_address, primary_output, total_fee)
                .unwrap();

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for
        // efficiency.
        //
        // Note: it doesn't matter which particular spending path to use, key or
        // script path, since the difference is only how the signature is
        // computed, which is a dummy signing function in our case.
        let signed_transaction = sign_transaction_key_spend(
            ctx,
            own_address,
            transaction.clone(),
            &prevouts,
            vec![], // mock derivation path
            vec![],
            mock_sign_with_schnorr,
        )
        .await;

        let tx_vsize = signed_transaction.vsize() as u64;
        if (tx_vsize * fee_per_byte) / 1000 == total_fee {
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
pub async fn sign_transaction_script_spend<SignFun, Fut>(
    ctx: &BitcoinContext,
    own_address: &Address,
    mut transaction: Transaction,
    prevouts: &[TxOut],
    control_block: &ControlBlock,
    script: &ScriptBuf,
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

        let leaf_hash = TapLeafHash::from_script(script, LeafVersion::TapScript);

        let signing_data = sighasher
            .taproot_script_spend_signature_hash(
                i,
                &bitcoin::sighash::Prevouts::All(prevouts),
                leaf_hash,
                TapSighashType::Default,
            )
            .expect("Failed to encode signing data")
            .as_byte_array()
            .to_vec();

        let raw_signature = signer(
            ctx.key_name.to_string(),
            derivation_path.clone(),
            None,
            signing_data.clone(),
        )
        .await;

        // Update the witness stack.

        let witness = sighasher.witness_mut(i).unwrap();
        witness.clear();
        let signature = bitcoin::taproot::Signature {
            signature: Signature::from_slice(&raw_signature).expect("failed to parse signature"),
            sighash_type: TapSighashType::Default,
        };
        witness.push(signature.to_vec());
        witness.push(script.to_bytes());
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
pub async fn sign_transaction_key_spend<SignFun, Fut>(
    ctx: &BitcoinContext,
    own_address: &Address,
    mut transaction: Transaction,
    prevouts: &[TxOut],
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
                &bitcoin::sighash::Prevouts::All(prevouts),
                TapSighashType::Default,
            )
            .expect("Failed to encode signing data")
            .as_byte_array()
            .to_vec();

        let raw_signature = signer(
            ctx.key_name.to_string(),
            derivation_path.clone(),
            Some(merkle_root_hash.clone()),
            signing_data.clone(),
        )
        .await;

        // Update the witness stack.
        let witness = sighasher.witness_mut(i).unwrap();
        let signature = bitcoin::taproot::Signature {
            signature: Signature::from_slice(&raw_signature).expect("failed to parse signature"),
            sighash_type: TapSighashType::Default,
        };
        witness.push(signature.to_vec());
    }

    transaction
}
