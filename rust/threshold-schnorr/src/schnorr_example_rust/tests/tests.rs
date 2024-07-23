use candid::{decode_one, encode_args, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, WasmResult};
use schnorr_example_rust::{
    PublicKeyReply, SchnorrAlgorithm, SignatureReply, SignatureVerificationReply,
};
use serde::Deserialize;
use std::path::Path;

#[test]
fn signing_and_verification_should_work_correctly() {
    const ALGORITHMS: [SchnorrAlgorithm; 2] =
        [SchnorrAlgorithm::Bip340Secp256k1, SchnorrAlgorithm::Ed25519];

    let pic = PocketIc::new();

    for algorithm in ALGORITHMS {
        for _trial in 0..5 {
            test_impl(&pic, algorithm);
        }
    }
}

fn test_impl(pic: &PocketIc, algorithm: SchnorrAlgorithm) {
    let my_principal = Principal::anonymous();

    // Create an empty example canister as the anonymous principal and add cycles.
    let example_canister_id = pic.create_canister();
    pic.add_cycles(example_canister_id, 2_000_000_000_000);

    let example_wasm_bytes = load_schnorr_example_canister_wasm();
    pic.install_canister(example_canister_id, example_wasm_bytes, vec![], None);

    // Make sure the canister is properly initialized
    fast_forward(&pic, 5);

    let message_hex = hex::encode("Test message");

    let sig_reply: Result<SignatureReply, String> = update(
        &pic,
        my_principal,
        example_canister_id,
        "sign",
        encode_args((message_hex.clone(), algorithm)).unwrap(),
    );

    let signature_hex = sig_reply.expect("failed to sign").signature_hex;

    let pk_reply: Result<PublicKeyReply, String> = update(
        &pic,
        my_principal,
        example_canister_id,
        "public_key",
        encode_one(algorithm).unwrap(),
    );

    let public_key_hex = pk_reply.unwrap().public_key_hex;

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message_hex.clone(),
                public_key_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert!(verification_reply.unwrap().is_signature_valid);
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                clone_and_reverse_chars(&signature_hex),
                message_hex.clone(),
                public_key_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert!(!verification_reply.unwrap().is_signature_valid);
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                clone_and_reverse_chars(&message_hex),
                public_key_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );

        assert!(!verification_reply.unwrap().is_signature_valid);
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message_hex.clone(),
                clone_and_reverse_chars(&public_key_hex),
                algorithm,
            ))
            .unwrap(),
        );

        assert!(
            verification_reply.is_err() || !verification_reply.unwrap().is_signature_valid,
            "either the public key should fail to deserialize or the verification should fail"
        );
    }

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                signature_hex.clone(),
                message_hex.clone(),
                public_key_hex.clone(),
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
        Ok(WasmResult::Reply(data)) => {
            decode_one(&data).map_err(|e| format!("failed to decode reply: {e:?}"))?
        }
        Ok(WasmResult::Reject(error_message)) => {
            Err(format!("canister rejected the message: {error_message}"))
        }
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
