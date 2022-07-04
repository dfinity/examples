use crate::types::*;
use crate::{bitcoin_api, ecdsa_api};
use bitcoin::util::psbt::serialize::Serialize as _;
use bitcoin::{
    blockdata::script::Builder, hashes::Hash, Address, AddressType, OutPoint, Script, SigHashType,
    Transaction, TxIn, TxOut, Txid,
};
use ic_btc_types::{Network, Utxo};
use ic_cdk::{print, trap};
use ic_cdk_macros::update;
use std::str::FromStr;

const DERIVATION_PATH: &[&[u8]] = &[&[0]];

/// Returns the P2PKH address of this canister at derivation path [0].
#[update]
async fn get_p2pkh_address() -> String {
    // Fetch our public key.
    let public_key = ecdsa_api::ecdsa_public_key(DERIVATION_PATH).await;

    // Compute the P2PKH address from our public key.
    crate::util::p2pkh_address_from_public_key(public_key)
}

#[update]
pub async fn send(request: SendRequest) {
    let amount = request.amount_in_satoshi;
    let destination_address = request.destination_address;

    let fee_percentiles = bitcoin_api::get_current_fee_percentiles().await;

    // Choose the 75th percentiles for sending fees.
    let fees = fee_percentiles[74];

    print!("Fee: {}", fees);

    if amount <= fees {
        trap(&format!(
            "Amount must be higher than the fee of {} satoshis",
            fees,
        ));
    }

    let our_address = get_p2pkh_address().await;

    // Fetch our UTXOs.
    let utxos = bitcoin_api::get_utxos(our_address.clone()).await.utxos;

    let spending_transaction = build_transaction(
        utxos,
        our_address.clone(),
        destination_address,
        amount,
        fees,
    )
    .expect("Error building transaction.");

    let tx_bytes = spending_transaction.serialize();
    print(&format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign transaction
    let signed_transaction = sign_transaction(
        spending_transaction,
        Address::from_str(&our_address).unwrap(),
        crate::ecdsa_api::ecdsa_public_key(DERIVATION_PATH).await,
    )
    .await;

    let signed_transaction_bytes = signed_transaction.serialize();
    print(&format!(
        "Signed transaction: {}",
        hex::encode(signed_transaction_bytes.clone())
    ));

    print("Sending transaction...");
    bitcoin_api::send_transaction(signed_transaction_bytes).await;
    print("Done");
}

// The signature hash type that is always used.
const SIG_HASH_TYPE: SigHashType = SigHashType::All;

// Builds a transaction that sends the given `amount` of satoshis to the `destination` address.
pub fn build_transaction(
    utxos: Vec<Utxo>,
    source: String,
    destination: String,
    amount: u64,
    fees: u64,
) -> Result<Transaction, String> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 10_000;

    let source = Address::from_str(&source).expect("Invalid destination address.");
    let destination = Address::from_str(&destination).expect("Invalid destination address.");

    // Select which UTXOs to spend. For now, we naively spend the first available UTXOs,
    // even if they were previously spent in a transaction.
    let mut utxos_to_spend = vec![];
    let mut total_spent = 0;
    for utxo in utxos.into_iter().rev() {
        total_spent += utxo.value;
        utxos_to_spend.push(utxo);
        if total_spent >= amount + fees {
            // We have enough inputs to cover the amount we want to spend.
            break;
        }
    }

    print(&format!("UTXOs to spend: {:?}", utxos_to_spend));
    print(&format!(
        "UTXO transaction id: {}",
        Txid::from_hash(Hash::from_slice(&utxos_to_spend[0].outpoint.txid).unwrap()),
    ));

    if total_spent < amount {
        return Err("Insufficient balance".to_string());
    }

    let inputs: Vec<TxIn> = utxos_to_spend
        .into_iter()
        .map(|utxo| TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                vout: utxo.outpoint.vout,
            },
            sequence: 0xffffffff,
            witness: Vec::new(),
            script_sig: Script::new(),
        })
        .collect();

    let mut outputs = vec![TxOut {
        script_pubkey: destination.script_pubkey(),
        value: amount,
    }];

    let remaining_amount = total_spent - amount - fees;

    if remaining_amount >= DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: source.script_pubkey(),
            value: remaining_amount,
        });
    }

    Ok(Transaction {
        input: inputs,
        output: outputs,
        lock_time: 0,
        version: 2,
    })
}

/// Sign a bitcoin transaction given the private key and the source address of the funds.
///
/// Constraints:
/// * All the inputs are referencing outpoints that are owned by `src_address`.
/// * `src_address` is a P2PKH address.
pub async fn sign_transaction(
    mut transaction: Transaction,
    src_address: Address,
    public_key: Vec<u8>,
) -> Transaction {
    // Verify that the address is P2PKH. The signature algorithm below is specific to P2PKH.
    match src_address.address_type() {
        Some(AddressType::P2pkh) => {}
        _ => panic!("This demo supports signing p2pkh addresses only."),
    };

    let txclone = transaction.clone();

    for (index, input) in transaction.input.iter_mut().enumerate() {
        let sighash =
            txclone.signature_hash(index, &src_address.script_pubkey(), SIG_HASH_TYPE.as_u32());

        let signature = crate::ecdsa_api::sign_with_ecdsa(sighash.to_vec()).await;

        // Convert signature to DER.
        let der_signature = crate::util::sec1_to_der(signature);

        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.as_u32() as u8);
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype.as_slice())
            .push_slice(public_key.as_slice())
            .into_script();
        input.witness.clear();
    }

    transaction
}
