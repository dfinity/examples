use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

const SIGN_WITH_SCHNORR_FEE: u128 = 25_000_000_000;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchnorrAlgorithm {
    #[serde(rename = "bip340secp256k1")]
    Bip340Secp256k1,
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
struct SchnorrKeyId {
    pub name: String,
    pub algorithm: SchnorrAlgorithm,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
struct SchnorrPublicKey {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SchnorrPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
struct SignWithSchnorr {
    pub message: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignWithSchnorrReply {
    pub signature: Vec<u8>,
}

/// Returns the Schnorr public key of this canister at the given derivation path.
pub async fn schnorr_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    let res: Result<(SchnorrPublicKeyReply,), _> = ic_cdk::call(
        Principal::management_canister(),
        "schnorr_public_key",
        (SchnorrPublicKey {
            canister_id: None,
            derivation_path,
            key_id: SchnorrKeyId {
                name: key_name,
                algorithm: SchnorrAlgorithm::Bip340Secp256k1,
            },
        },),
    )
    .await;

    res.unwrap().0.public_key
}

pub async fn sign_with_schnorr(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message: Vec<u8>,
) -> Vec<u8> {
    let res: Result<(SignWithSchnorrReply,), _> = ic_cdk::api::call::call_with_payment128(
        Principal::management_canister(),
        "sign_with_schnorr",
        (SignWithSchnorr {
            message,
            derivation_path,
            key_id: SchnorrKeyId {
                name: key_name,
                algorithm: SchnorrAlgorithm::Bip340Secp256k1,
            },
        },),
        SIGN_WITH_SCHNORR_FEE,
    )
    .await;

    res.unwrap().0.signature
}
