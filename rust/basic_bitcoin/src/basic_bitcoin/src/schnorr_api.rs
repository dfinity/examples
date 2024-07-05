use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::SCHNORR_CANISTER;

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
    let canister_id = SCHNORR_CANISTER.with(|schnorr_canister| {
        ic_cdk::println!(
            "CANISTER_ID_SCHNORR_CANISTER: {:?}",
            &schnorr_canister.borrow()
        );

        Principal::from_text(schnorr_canister.borrow().as_str()).unwrap()
    });

    let res: Result<(SchnorrPublicKeyReply,), _> = ic_cdk::call(
        canister_id,
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
    let canister_id = SCHNORR_CANISTER
        .with(|schnorr_canister| Principal::from_text(schnorr_canister.borrow().as_str()).unwrap());

    let res: Result<(SignWithSchnorrReply,), _> = ic_cdk::call(
        canister_id,
        "sign_with_schnorr",
        (SignWithSchnorr {
            message,
            derivation_path,
            key_id: SchnorrKeyId {
                name: key_name,
                algorithm: SchnorrAlgorithm::Bip340Secp256k1,
            },
        },),
    )
    .await;

    res.unwrap().0.signature
}
