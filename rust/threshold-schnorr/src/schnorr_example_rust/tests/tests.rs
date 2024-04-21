use candid::{decode_one, encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, WasmResult};
use schnorr_example_rust::{
    PublicKeyReply, SchnorrAlgorithm, SignatureReply, SignatureRequest, SignatureVerificationReply,
};
use serde::Deserialize;
use std::path::Path;

#[test]
fn signing_and_verification_should_work_correctly() {
    const ALGORITHMS: [SchnorrAlgorithm; 2] =
        [SchnorrAlgorithm::Bip340Secp256k1, SchnorrAlgorithm::Ed25519];

    for algorithm in ALGORITHMS {
        test_impl(algorithm);
    }
}

fn test_impl(algorithm: SchnorrAlgorithm) {
    let pic = PocketIc::new();

    let my_principal = Principal::anonymous();
    // Create an empty canister as the anonymous principal and add cycles.
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm_bytes = load_schnorr_canister_wasm();
    pic.install_canister(canister_id, wasm_bytes, vec![], None);

    // Make sure the canister is properly initialized
    fast_forward(&pic, 5);

    let message_hex = hex::encode("Test message");
    let signature_request = SignatureRequest {
        message: message_hex.clone(),
        algorithm: algorithm,
    };

    let sig_reply: Result<SignatureReply, String> = update(
        &pic,
        my_principal,
        canister_id,
        "sign",
        encode_one(signature_request).unwrap(),
    );

    let signature_hex = sig_reply.expect("failed to sign").signature_hex;

    let pk_reply: Result<PublicKeyReply, String> = update(
        &pic,
        my_principal,
        canister_id,
        "public_key",
        encode_one(algorithm).unwrap(),
    );

    let public_key_hex = pk_reply.unwrap().public_key_hex;

    {
        let verification_reply: Result<SignatureVerificationReply, String> = update(
            &pic,
            my_principal,
            canister_id,
            "verify",
            encode_one((
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
            canister_id,
            "verify",
            encode_one((
                clone_and_swap_two_first_chars(&signature_hex),
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
            canister_id,
            "verify",
            encode_one((
                signature_hex.clone(),
                clone_and_swap_two_first_chars(&message_hex),
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
            canister_id,
            "verify",
            encode_one((
                signature_hex.clone(),
                message_hex.clone(),
                clone_and_swap_two_first_chars(&public_key_hex),
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
            canister_id,
            "verify",
            encode_one((
                signature_hex.clone(),
                message_hex.clone(),
                public_key_hex.clone(),
                other_algorithm(algorithm),
            ))
            .unwrap(),
        );

        assert!(!verification_reply.unwrap().is_signature_valid);
    }
}

fn clone_and_swap_two_first_chars(s: &str) -> String {
    let mut v: Vec<_> = s.chars().collect();
    v.swap(0, 1);
    v.into_iter().collect()
}

fn load_schnorr_canister_wasm() -> Vec<u8> {
    let wasm_path =
        Path::new("../../target/wasm32-unknown-unknown/release/schnorr_example_rust.wasm");
    std::fs::read(wasm_path).unwrap()
}

pub fn update<T: CandidType + for<'de> Deserialize<'de>>(
    ic: &PocketIc,
    sender: Principal,
    receiver: Principal,
    method: &str,
    args: Vec<u8>,
) -> Result<T, String> {
    match ic.update_call(receiver, sender, method, args) {
        Ok(WasmResult::Reply(data)) => {
            decode_one(&data).map_err(|e| format!("failed to decode reply: {e:?}"))?
        }
        Ok(WasmResult::Reject(error_message)) => {
            Err(format!("canister rejected the message: {error_message}"))
        }
        Err(user_error) => Err(format!("canister returned a user error: {user_error}")),
    }
}

pub fn fast_forward(ic: &PocketIc, ticks: u64) {
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
