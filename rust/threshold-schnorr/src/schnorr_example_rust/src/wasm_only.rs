use super::{PublicKeyReply, SchnorrAlgorithm, SignatureReply, SignatureVerificationReply};
use bitcoin::{
    key::{Secp256k1, TapTweak},
    XOnlyPublicKey,
};
use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::convert::TryInto;

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ManagementCanisterSchnorrPublicKeyRequest {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ManagementCanisterSchnorrPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct SchnorrKeyId {
    pub algorithm: SchnorrAlgorithm,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug)]
struct ManagementCanisterSignatureRequest {
    pub message: Vec<u8>,
    pub aux: Option<SignWithSchnorrAux>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(Eq, PartialEq, Debug, CandidType, Serialize)]
pub enum SignWithSchnorrAux {
    #[serde(rename = "bip341")]
    Bip341(SignWithBip341Aux),
}

#[derive(Eq, PartialEq, Debug, CandidType, Serialize)]
pub struct SignWithBip341Aux {
    pub merkle_root_hash: ByteBuf,
}

#[derive(CandidType, Deserialize, Debug)]
struct ManagementCanisterSignatureReply {
    pub signature: Vec<u8>,
}

thread_local! {
    static STATE: RefCell<String> = RefCell::new("aaaaa-aa".to_string());
}

#[update]
async fn for_test_only_change_management_canister_id(id: String) -> Result<(), String> {
    let _ = CanisterId::from_text(&id).map_err(|e| panic!("invalid canister id: {}: {}", id, e));
    STATE.with_borrow_mut(move |current_id| {
        println!(
            "Changing management canister id from {} to {id}",
            *current_id
        );
        *current_id = id;
    });
    Ok(())
}

#[update]
async fn public_key(algorithm: SchnorrAlgorithm) -> Result<PublicKeyReply, String> {
    let request = ManagementCanisterSchnorrPublicKeyRequest {
        canister_id: None,
        derivation_path: vec![ic_cdk::api::caller().as_slice().to_vec()],
        key_id: SchnorrKeyIds::TestKeyLocalDevelopment.to_key_id(algorithm),
    };

    let (res,): (ManagementCanisterSchnorrPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "schnorr_public_key", (request,))
            .await
            .map_err(|e| format!("schnorr_public_key failed {}", e.1))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&res.public_key),
    })
}

#[update]
async fn sign(
    message: String,
    algorithm: SchnorrAlgorithm,
    opt_merkle_tree_root_hex: Option<String>,
) -> Result<SignatureReply, String> {
    let aux = opt_merkle_tree_root_hex
        .map(|hex| {
            hex::decode(&hex)
                .map_err(|e| format!("failed to decode hex: {e:?}"))
                .and_then(|bytes| {
                    if bytes.len() == 32 || bytes.is_empty() {
                        Ok(SignWithSchnorrAux::Bip341(SignWithBip341Aux {
                            merkle_root_hash: ByteBuf::from(bytes),
                        }))
                    } else {
                        Err(format!(
                            "merkle tree root bytes must be 0 or 32 bytes long but got {}",
                            bytes.len()
                        ))
                    }
                })
        })
        .transpose()?;

    let internal_request = ManagementCanisterSignatureRequest {
        message: message.as_bytes().to_vec(),
        derivation_path: vec![ic_cdk::api::caller().as_slice().to_vec()],
        key_id: SchnorrKeyIds::TestKeyLocalDevelopment.to_key_id(algorithm),
        aux,
    };

    let (internal_reply,): (ManagementCanisterSignatureReply,) =
        ic_cdk::api::call::call_with_payment(
            mgmt_canister_id(),
            "sign_with_schnorr",
            (internal_request,),
            26_153_846_153,
        )
        .await
        .map_err(|e| format!("sign_with_schnorr failed {e:?}"))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&internal_reply.signature),
    })
}

#[query]
async fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
    opt_merkle_tree_root_hex: Option<String>,
    algorithm: SchnorrAlgorithm,
) -> Result<SignatureVerificationReply, String> {
    let sig_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let msg_bytes = message.as_bytes();
    let pk_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");

    match algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => match opt_merkle_tree_root_hex {
            Some(merkle_tree_root_hex) => {
                let merkle_tree_root_bytes = hex::decode(&merkle_tree_root_hex)
                    .expect("failed to hex-decode merkle tree root");
                verify_bip341_secp256k1(&sig_bytes, msg_bytes, &pk_bytes, &merkle_tree_root_bytes)
            }
            None => verify_bip340_secp256k1(&sig_bytes, msg_bytes, &pk_bytes),
        },
        SchnorrAlgorithm::Ed25519 => {
            if let Some(_) = opt_merkle_tree_root_hex {
                return Err("ed25519 does not support merkle tree root verification".to_string());
            }
            verify_ed25519(&sig_bytes, &msg_bytes, &pk_bytes)
        }
    }
}

fn verify_bip340_secp256k1(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    secp1_pk_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    assert_eq!(secp1_pk_bytes.len(), 33);

    let sig = bitcoin::secp256k1::schnorr::Signature::from_slice(sig_bytes)
        .expect("failed to deserialize signature");

    let pk = bitcoin::secp256k1::XOnlyPublicKey::from_slice(&secp1_pk_bytes[1..])
        .expect("failed to deserialize BIP340 encoding into public key");

    let secp256k1_engine = Secp256k1::new();
    let msg =
        bitcoin::secp256k1::Message::from_digest_slice(msg_bytes).expect("failed to parse message");
    let is_signature_valid = pk.verify(&secp256k1_engine, &msg, &sig).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

fn verify_bip341_secp256k1(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    secp1_pk_bytes: &[u8],
    merkle_tree_root_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    assert_eq!(secp1_pk_bytes.len(), 33);

    let pk = XOnlyPublicKey::from_slice(&secp1_pk_bytes[1..]).unwrap();
    let tweaked_pk_bytes = {
        let secp256k1_engine = Secp256k1::new();
        let merkle_root = if merkle_tree_root_bytes.len() == 0 {
            None
        } else {
            Some(
                bitcoin::hashes::Hash::from_slice(&merkle_tree_root_bytes)
                    .expect("failed to create TapBranchHash"),
            )
        };

        pk.tap_tweak(&secp256k1_engine, merkle_root)
            .0
            .to_inner()
            .serialize()
    };

    let sig = bitcoin::secp256k1::schnorr::Signature::from_slice(sig_bytes)
        .expect("failed to deserialize signature");

    let pk = bitcoin::secp256k1::XOnlyPublicKey::from_slice(&tweaked_pk_bytes)
        .expect("failed to deserialize tweaked BIP340 encoding into public key");

    let secp256k1_engine = Secp256k1::new();
    let msg =
        bitcoin::secp256k1::Message::from_digest_slice(msg_bytes).expect("failed to parse message");
    let is_signature_valid = pk.verify(&secp256k1_engine, &msg, &sig).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

fn verify_ed25519(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    pk_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let pk: [u8; 32] = pk_bytes
        .try_into()
        .expect("ed25519 public key incorrect length");
    let vk = VerifyingKey::from_bytes(&pk).expect("failed to parse ed25519 public key");

    let signature = Signature::from_slice(sig_bytes).expect("ed25519 signature incorrect length");

    let is_signature_valid = vk.verify(msg_bytes, &signature).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

enum SchnorrKeyIds {
    #[allow(unused)]
    ChainkeyTestingCanisterKey1,
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl SchnorrKeyIds {
    fn to_key_id(&self, algorithm: SchnorrAlgorithm) -> SchnorrKeyId {
        SchnorrKeyId {
            algorithm,
            name: match self {
                Self::ChainkeyTestingCanisterKey1 => "insecure_test_key_1",
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

fn mgmt_canister_id() -> CanisterId {
    STATE.with_borrow(|state| CanisterId::from_text(&state).unwrap())
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
