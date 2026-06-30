use backend::{PublicKeyReply, SchnorrAlgorithm, SignatureReply, SignatureVerificationReply};
use candid::{decode_one, encode_args, encode_one};
use candid::{CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};
use serde::Deserialize;

/// Tests every combination of algorithm (BIP340-secp256k1, Ed25519) and merkle root hash
/// (absent, empty, invalid length, valid 32 bytes) for sign+verify, plus negative cases.
///
/// The PocketIC instance is reused across iterations for speed.
/// The fiduciary subnet is required for threshold signing operations.
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

    // BIP341 signing is only valid with an absent merkle root, an empty one (no script path),
    // or a 32-byte merkle root. An 8-byte value is intentionally invalid.
    let should_validate = (merkle_tree_root_bytes
        .as_ref()
        .map(|v| v.is_empty() || v.len() == 32)
        != Some(false)
        && algorithm == SchnorrAlgorithm::Bip340Secp256k1)
        || merkle_tree_root_bytes.is_none();

    let merkle_tree_root_hex = merkle_tree_root_bytes.map(hex::encode);

    let example_canister_id = pic.create_canister();
    pic.add_cycles(example_canister_id, 2_000_000_000_000);
    pic.install_canister(example_canister_id, load_backend_wasm(), vec![], None);

    // PocketIC's fiduciary subnet provides "key_1", not "dfx_test_key" (icp local network).
    update::<Result<(), String>>(
        pic,
        my_principal,
        example_canister_id,
        "for_test_only_set_schnorr_key_name",
        encode_one("key_1").unwrap(),
    )
    .expect("update call failed")
    .expect("set key name failed");

    // Let the canister initialise before making calls.
    fast_forward(pic, 5);

    // Use a 32-byte message so it is valid for all three algorithms.
    // BIP340 requires exactly 32 bytes; ed25519 accepts any length.
    let message: String = std::iter::repeat('a')
        .take(16)
        .chain(std::iter::repeat('b').take(16))
        .collect();

    let public_key_hex = update::<Result<PublicKeyReply, String>>(
        pic,
        my_principal,
        example_canister_id,
        "public_key",
        encode_one(algorithm).unwrap(),
    )
    .expect("update call failed")
    .unwrap()
    .public_key_hex;

    // The outer Result is the network/call result; the inner is the canister's return value.
    // For negative cases the canister may either return Err or trap (rejected call).
    // Both are != Ok(Ok(SignatureVerificationReply { is_signature_valid: true })).
    let verified = Ok(Ok::<SignatureVerificationReply, String>(
        SignatureVerificationReply { is_signature_valid: true },
    ));

    let sig_reply: Result<SignatureReply, String> = update(
        pic,
        my_principal,
        example_canister_id,
        "sign",
        encode_args((message.clone(), algorithm, merkle_tree_root_hex.clone())).unwrap(),
    )
    .expect("update call failed");

    if sig_reply.is_err() {
        // Signing should only fail for invalid parameters (e.g. 8-byte merkle root for BIP341).
        assert!(
            !should_validate,
            "signing unexpectedly failed for valid params: algorithm={algorithm:?}, merkle_root={merkle_tree_root_hex:?}"
        );
        // Even with a dummy signature, verify should reject it.
        let dummy_sig = "a".repeat(128);
        let verify_reply = update::<Result<SignatureVerificationReply, String>>(
            pic,
            my_principal,
            example_canister_id,
            "verify",
            encode_args((
                dummy_sig,
                message.clone(),
                public_key_hex.clone(),
                merkle_tree_root_hex.clone(),
                algorithm,
            ))
            .unwrap(),
        );
        assert_ne!(verify_reply, verified);
        return;
    }

    let signature_hex = sig_reply.unwrap().signature_hex;

    // Valid signature verifies.
    {
        let reply = update::<Result<SignatureVerificationReply, String>>(
            pic,
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
        )
        .expect("verify update call unexpectedly failed");

        if should_validate {
            assert_eq!(reply, Ok(SignatureVerificationReply { is_signature_valid: true }), "valid signature should verify");
        } else {
            assert_ne!(reply, Ok(SignatureVerificationReply { is_signature_valid: true }));
        }
    }

    // Corrupted signature does not verify (canister may return Err or trap).
    assert_ne!(
        update::<Result<SignatureVerificationReply, String>>(
            pic, my_principal, example_canister_id, "verify",
            encode_args((reverse_chars(&signature_hex), message.clone(), public_key_hex.clone(), merkle_tree_root_hex.clone(), algorithm)).unwrap(),
        ),
        verified,
    );

    // Corrupted message does not verify.
    assert_ne!(
        update::<Result<SignatureVerificationReply, String>>(
            pic, my_principal, example_canister_id, "verify",
            encode_args((signature_hex.clone(), reverse_chars(&message), public_key_hex.clone(), merkle_tree_root_hex.clone(), algorithm)).unwrap(),
        ),
        verified,
    );

    // Corrupted public key does not verify (canister traps on invalid key bytes).
    assert_ne!(
        update::<Result<SignatureVerificationReply, String>>(
            pic, my_principal, example_canister_id, "verify",
            encode_args((signature_hex.clone(), message.clone(), reverse_chars(&public_key_hex), merkle_tree_root_hex.clone(), algorithm)).unwrap(),
        ),
        verified,
    );

    // Wrong algorithm: key sizes differ between BIP340 and Ed25519 so the canister traps.
    assert_ne!(
        update::<Result<SignatureVerificationReply, String>>(
            pic, my_principal, example_canister_id, "verify",
            encode_args((signature_hex.clone(), message.clone(), public_key_hex.clone(), merkle_tree_root_hex.clone(), other_algorithm(algorithm))).unwrap(),
        ),
        verified,
    );
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn load_backend_wasm() -> Vec<u8> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    // CARGO_MANIFEST_DIR is the `backend/` crate; the workspace target dir is one level up.
    let wasm_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/wasm32-unknown-unknown/release/backend.wasm");

    let wasm_bytes = std::fs::read(&wasm_path).unwrap_or_else(|_| {
        panic!(
            "WASM not found at {}. Run: cargo build --package backend --target wasm32-unknown-unknown --release",
            wasm_path.display()
        )
    });

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&wasm_bytes).unwrap();
    encoder.finish().unwrap()
}

/// Calls an update method and decodes the Candid-encoded reply.
fn update<T: CandidType + for<'de> Deserialize<'de>>(
    ic: &PocketIc,
    sender: Principal,
    canister_id: Principal,
    method: &str,
    args: Vec<u8>,
) -> Result<T, String> {
    match ic.update_call(canister_id, sender, method, args) {
        Ok(data) => decode_one(&data).map_err(|e| format!("failed to decode reply: {e:?}")),
        Err(reject) => Err(format!("canister rejected call: {reject:?}")),
    }
}

fn fast_forward(ic: &PocketIc, ticks: u64) {
    for _ in 0..ticks {
        ic.tick();
    }
}

fn reverse_chars(s: &str) -> String {
    s.chars().rev().collect()
}

fn other_algorithm(algorithm: SchnorrAlgorithm) -> SchnorrAlgorithm {
    match algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => SchnorrAlgorithm::Ed25519,
        SchnorrAlgorithm::Ed25519 => SchnorrAlgorithm::Bip340Secp256k1,
    }
}
