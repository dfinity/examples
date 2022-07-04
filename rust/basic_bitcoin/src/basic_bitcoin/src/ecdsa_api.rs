use crate::types::*;
use ic_cdk::{call, export::Principal};

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn ecdsa_public_key(derivation_path: &[&[u8]]) -> Vec<u8> {
    // TODO: pass in the full vec.
    // TODO: Replace this principal with the management canister when it's available.
    //       For now, call a canister that provides a mock implementation.
    let ecdsa_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    // Retrieve the public key of this canister at derivation path [0]
    // from the ECDSA API.
    let res: (ECDSAPublicKeyReply,) = call(
        ecdsa_canister_id,
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id: None,
            derivation_path: derivation_path
                .to_vec()
                .iter()
                .map(|e| e.to_vec())
                .collect(),
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: String::from("test"),
            },
        },),
    )
    .await
    .unwrap();

    res.0.public_key
}

pub async fn sign_with_ecdsa(message_hash: Vec<u8>) -> Vec<u8> {
    // TODO: pass in the full derivation vec.
    // TODO: Replace this principal with the management canister when it's available.
    //       For now, call a canister that provides a mock implementation.
    let ecdsa_canister_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    let res: (SignWithECDSAReply,) = call(
        ecdsa_canister_id,
        "sign_with_ecdsa",
        (crate::SignWithECDSA {
            message_hash,
            derivation_path: vec![vec![0]],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: String::from("test"),
            },
        },),
    )
    .await
    .unwrap();

    res.0.signature
}
