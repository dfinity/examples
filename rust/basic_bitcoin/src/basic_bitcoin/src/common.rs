use crate::types::*;
use bitcoin::{
    blockdata::script::Builder,
    hashes::Hash,
    Address, AddressType, OutPoint, Script, SigHashType, Transaction, TxIn,
    TxOut, Txid,
};
use ic_btc_types::Utxo;
use ic_cdk::{
    call,
    export::{Principal},
    print,
};

// The signature hash type that is always used.
const SIG_HASH_TYPE: SigHashType = SigHashType::All;

// Builds a transaction that sends the given `amount` of satoshis to the `destination` address.
pub fn build_transaction(
    utxos: Vec<Utxo>,
    source: Address,
    destination: Address,
    amount: u64,
    fees: u64,
) -> Result<Transaction, String> {
    // Assume that any amount below this threshold is dust.
    const DUST_THRESHOLD: u64 = 10_000;

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
        let ecdsa_canister_id = Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();

        let res: (SignWithECDSAReply,) = call(
            ecdsa_canister_id,
            "sign_with_ecdsa",
            (crate::SignWithECDSA {
                message_hash: sighash.to_vec(),
                derivation_path: vec![vec![0]],
                key_id: EcdsaKeyId {
                    curve: EcdsaCurve::Secp256k1,
                    name: String::from("test"),
                },
            },),
        )
        .await
        .unwrap();

        let signature = res.0.signature;
        let r: Vec<u8> = if signature[0] & 0x80 != 0 {
            // r is negative. Prepend a zero byte.
            let mut tmp = vec![0x00];
            tmp.extend(signature[..32].to_vec());
            tmp
        } else {
            // r is positive.
            signature[..32].to_vec()
        };

        let s: Vec<u8> = if signature[32] & 0x80 != 0 {
            // s is negative. Prepend a zero byte.
            let mut tmp = vec![0x00];
            tmp.extend(signature[32..].to_vec());
            tmp
        } else {
            // s is positive.
            signature[32..].to_vec()
        };

        // Convert signature to DER.
        let der_signature: Vec<u8> = vec![
            vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
            r,
            vec![0x02, s.len() as u8],
            s,
        ]
        .into_iter()
        .flatten()
        .collect();

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

#[test]
fn test_sign_transaction() {
    tokio_test::block_on(async {
        use bitcoin::secp256k1::rand::rngs::OsRng;
        use bitcoin::{Network, OutPoint, PublicKey, Script, Transaction, TxIn, TxOut};

        // Generate an address.
        let mut rng = OsRng::new().unwrap();
        let secp = Secp256k1::new();
        let public_key =
            hex::decode("02053d28d6abb9fbf9fd37fec1d32e6ae46ee2e3cff5d77991855422215ccd6362")
                .unwrap();

        let address = get_address_from_public_key(public_key);
        println!("Address: {}", address);

        let (private_key, public_key) = secp.generate_keypair(&mut rng);
        //let public_key = PublicKey::new(public_key);
        let private_key = PrivateKey::new(private_key, Network::Regtest);
        let address = Address::from_str(&address).unwrap();

        let spending_transaction = Transaction {
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::Txid::default(),
                    vout: 0,
                },
                sequence: 0xffffffff,
                witness: Vec::new(),
                script_sig: Script::new(),
            }],
            output: vec![TxOut {
                script_pubkey: address.script_pubkey(),
                value: 99,
            }],
            lock_time: 0,
            version: 2,
        };

        let spending_transaction =
            sign_transaction(spending_transaction, address.clone(), vec![]).await;

        use bitcoin::util::psbt::serialize::Serialize;
        println!(
            "raw signed transaction: {}",
            hex::encode(spending_transaction.serialize())
        );
        //        assert_eq!(
        // Use the `bitcoinconsensus` lib to verify the correctness of the transaction.
        spending_transaction
            .verify(|_outpoint| {
                Some(TxOut {
                    value: 100,
                    script_pubkey: address.script_pubkey(),
                })
            })
            .map_err(|err| err.to_string())
            .unwrap();
        //            Ok(())
        //      );
    });
}
