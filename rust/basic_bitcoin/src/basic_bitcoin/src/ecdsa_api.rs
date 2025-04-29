use ic_cdk::management_canister::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs,
    SignWithEcdsaArgs,
};

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn get_ecdsa_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let canister_id = None;
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: key_name,
    };

    let res = ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id,
        derivation_path,
        key_id,
    })
    .await;

    res.unwrap().public_key
}

pub async fn get_ecdsa_signature(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Vec<u8> {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: key_name,
    };

    let res = sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash,
        derivation_path,
        key_id,
    })
    .await;

    res.unwrap().signature
}
