use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::{query, update};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(CandidType, Serialize, Debug)]
struct PublicKeyReply {
    pub public_key_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
struct SignatureReply {
    pub signature_hex: String,
}

#[derive(CandidType, Serialize, Debug)]
struct SignatureVerificationReply {
    pub is_signature_valid: bool,
}

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignWithECDSAReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[update]
async fn public_key() -> Result<PublicKeyReply, String> {
    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ECDSAPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "ecdsa_public_key", (request,))
            .await
            .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&res.public_key),
    })
}

#[update]
async fn sign(message: String) -> Result<SignatureReply, String> {
    let request = SignWithECDSA {
        message_hash: sha256(&message).to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,): (SignWithECDSAReply,) = ic_cdk::api::call::call_with_payment(
        mgmt_canister_id(),
        "sign_with_ecdsa",
        (request,),
        25_000_000_000,
    )
    .await
    .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&response.signature),
    })
}

#[query]
async fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
) -> Result<SignatureVerificationReply, String> {
    let signature_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let pubkey_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");
    let message_bytes = message.as_bytes();

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .expect("failed to deserialize signature");
    let is_signature_valid= k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .expect("failed to deserialize sec1 encoding into public key")
        .verify(message_bytes, &signature)
        .is_ok();

    Ok(SignatureVerificationReply{
        is_signature_valid
    })
}

fn mgmt_canister_id() -> CanisterId {
    CanisterId::from_str(&"aaaaa-aa").unwrap()
}

fn sha256(input: &String) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}

enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl EcdsaKeyIds {
    fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
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
