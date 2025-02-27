use candid::{decode_one, encode_args, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use schnorr_example_rust::{
    PublicKeyReply, SchnorrAlgorithm, SignatureReply, SignatureVerificationReply,
};
use serde::Deserialize;
use std::path::Path;

#[test]
fn signing_and_verification_should_work_correctly() {
    const ALGORITHMS: [SchnorrAlgorithm; 2] =
        [SchnorrAlgorithm::Bip340Secp256k1, SchnorrAlgorithm::Ed25519];
    let merkle_root_hashes: [Option<Vec<u8>>; 4] =
        [None, Some(vec![]), Some(vec![0; 8]), Some(vec![0; 32])];

    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_ii_subnet()
        .with_fiduciary_subnet()
        .build();

    for algorithm in ALGORITHMS {
        for merkle_root_hash in merkle_root_hashes.iter() {
            for _trial in 0..5 {
                test_impl(&pic, algorithm, merkle_root_hash.clone());
            }
        }
    }
}

fn test_impl(pic: &PocketIc, algorithm: SchnorrAlgorithm, merkle_tree_root_bytes: Option<Vec<u8>>) {
    let my_principal = Principal::anonymous();

    let should_validate = (merkle_tree_root_bytes
        .as_ref()
        .map(|v| v.len() == 0 || v.len() == 32)
        != Some(false)
        && algorithm == SchnorrAlgorithm::Bip340Secp256k1)
        || merkle_tree_root_bytes.is_none();

    let merkle_tree_root_hex = merkle_tree_root_bytes.map(|v| hex::encode(v));

    // Create an empty example canister as the anonymous principal and add cycles.
    let example_canister_id = pic.create_canister();
    pic.add_cycles(example_canister_id, 2_000_000_000_000);

    let example_wasm_bytes = load_schnorr_example_canister_wasm();
    pic.install_canister(example_canister_id, example_wasm_bytes, vec![], None);

    // Make sure the canister is properly initialized
    fast_forward(&pic, 5);

    // a message we can reverse to break the signature
    let message: String = std::iter::repeat('a')
        .take(16)
        .chain(std::iter::repeat('b').take(16))
        .collect();

    let pk_reply: Result<PublicKeyReply, String> = update(
        &pic,
        my_principal,
        example_canister_id,
        "public_key",
        encode_one(algorithm).unwrap(),
    );

    let public_key_hex = pk_reply.unwrap().public_key_hex;

    let successful_validation = Ok(SignatureVerificationReply {
        is_signature_valid: true,
    });

    let sig_reply: Result<SignatureReply, String> = update(
        &pic,
        my_principal,
        example_canister_id,
        "sign",
        encode_args((message.clone(), algorithm, merkle_tree_root_hex.clone())).unwrap(),
    );

    if sig_reply.is_err() {
        // If we failed to produce a signature with particular testing
        // parameters, still test that the verification fails on dummy inputs.
        assert!(!should_validate);
        let dummy_signature_hex = String::from("a".repeat(64));
        assert_ne!(
            update(
                &pic,
                my_principal,
                example_canister_id,
                "verify",
                encode_args((
                    dummy_signature_hex,
                    message.clone(),
                    public_key_hex.clone(),
                    merkle_tree_root_hex.clone(),
                    algorithm,
                ))
                .unwrap(),
            ),
            successful_validation.clone()
        );
        return;
    }

    let signature_hex = sig_reply.expect("failed to sign").signature_hex;

    {
        let verification_reply = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message.clone(),
                public_key_hex.clone(),
                merkle_tree_root_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        if should_validate {
            assert_eq!(verification_reply, successful_validation.clone());
        } else {
            assert_ne!(verification_reply, successful_validation.clone());
        }
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                clone_and_reverse_chars(&signature_hex),
                message.clone(),
                public_key_hex.clone(),
                merkle_tree_root_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert_ne!(verification_reply, successful_validation.clone());
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                clone_and_reverse_chars(&message),
                public_key_hex.clone(),
                merkle_tree_root_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert_ne!(verification_reply, successful_validation.clone());
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message.clone(),
                clone_and_reverse_chars(&public_key_hex),
                merkle_tree_root_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert_ne!(verification_reply, successful_validation.clone());
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message.clone(),
                public_key_hex.clone(),
                merkle_tree_root_hex.clone(),
                other_algorithm(algorithm),
            ))
            .unwrap(),
        );

        assert!(
            verification_reply.is_err(),
            "ed25519 and BIP340 should have different public key sizes"
        );
    }
}

fn clone_and_reverse_chars(s: &str) -> String {
    let mut v: Vec<_> = s.chars().collect();
    v.reverse();
    v.into_iter().collect()
}

fn load_schnorr_example_canister_wasm() -> Vec<u8> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::prelude::*;

    let wasm_path =
        Path::new("../../target/wasm32-unknown-unknown/release/schnorr_example_rust.wasm");
    let wasm_bytes = std::fs::read(wasm_path).expect(
        "wasm does not exist - run `cargo build --release --target wasm32-unknown-unknown`",
    );

    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(wasm_bytes.as_slice()).unwrap();
    let zipped_bytes = e.finish().unwrap();

    zipped_bytes
}

pub fn update<T: CandidType + for<'de> Deserialize<'de>>(
    ic: &PocketIc,
    sender: Principal,
    canister_id: Principal,
    method: &str,
    args: Vec<u8>,
) -> Result<T, String> {
    match ic.update_call(canister_id, sender, method, args) {
        Ok(data) => decode_one(&data).map_err(|e| format!("failed to decode reply: {e:?}"))?,
        Err(user_error) => Err(format!("canister returned a user error: {user_error}")),
    }
}

fn fast_forward(ic: &PocketIc, ticks: u64) {
    for _ in 0..ticks - 1 {
        ic.tick();
    }
}

fn other_algorithm(algorithm: SchnorrAlgorithm) -> SchnorrAlgorithm {
    match algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => SchnorrAlgorithm::Ed25519,
        SchnorrAlgorithm::Ed25519 => SchnorrAlgorithm::Bip340Secp256k1,
    }
}
