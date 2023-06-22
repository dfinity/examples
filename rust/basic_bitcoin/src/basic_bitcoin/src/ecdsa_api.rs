use ic_cdk::api::management_canister::ecdsa::{
    self, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse,
    SignWithEcdsaArgument, SignWithEcdsaResponse,
};

const SIGN_WITH_ECDSA_FEE: u128 = 26153846153;

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn ecdsa_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let res: (EcdsaPublicKeyResponse,) = ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path,
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: key_name,
        },
    })
    .await
    .unwrap();

    res.0.public_key
}

pub async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Vec<u8> {
    let res: (SignWithEcdsaResponse,) = ecdsa::sign_with_ecdsa(
        SignWithEcdsaArgument {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },
        SIGN_WITH_ECDSA_FEE,
    )
    .await
    .unwrap();

    res.0.signature
}
