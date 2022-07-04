use sha2::Digest;

pub fn p2pkh_address_from_public_key(public_key: Vec<u8>) -> String {
    // sha256 + ripmd160
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(sha256(public_key));
    let result = hasher.finalize();

    // mainnet: 0x00, testnet: 0x6f
    let mut data_with_prefix = vec![0x6f];
    data_with_prefix.extend(result);

    //let data_with_prefix_b58 = bs58::encode(data_with_prefix);
    // TODO: get rid of clone?
    let checksum = &sha256(sha256(data_with_prefix.clone()))[..4];

    let mut full_address = data_with_prefix;
    full_address.extend(checksum);

    bs58::encode(full_address).into_string()
}

pub fn sha256(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
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
