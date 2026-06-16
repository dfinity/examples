use candid::CandidType;
use ic_cdk::{query, update};
use ic_cdk_management_canister::{EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs, SignWithEcdsaArgs};
use serde::Serialize;
use std::convert::TryFrom;

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

// Use "test_key_1" for mainnet test key, "key_1" for mainnet production key,
// or "dfx_test_key" for local development.
const KEY_ID: EcdsaKeyIds = EcdsaKeyIds::TestKey1;

#[update]
async fn public_key() -> Result<PublicKeyReply, String> {
    let response = ic_cdk_management_canister::ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id: None,
        derivation_path: vec![],
        key_id: KEY_ID.to_key_id(),
    })
    .await
    .map_err(|e| format!("ecdsa_public_key failed: {:?}", e))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(response.public_key),
    })
}

#[update]
async fn sign(message: String) -> Result<SignatureReply, String> {
    let response = ic_cdk_management_canister::sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash: sha256(&message).to_vec(),
        derivation_path: vec![],
        key_id: KEY_ID.to_key_id(),
    })
    .await
    .map_err(|e| format!("sign_with_ecdsa failed: {:?}", e))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(response.signature),
    })
}

#[query]
fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
) -> Result<SignatureVerificationReply, String> {
    let signature_bytes =
        hex::decode(&signature_hex).map_err(|e| format!("failed to hex-decode signature: {e}"))?;
    let pubkey_bytes = hex::decode(&public_key_hex)
        .map_err(|e| format!("failed to hex-decode public key: {e}"))?;
    let message_bytes = message.as_bytes();

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| format!("failed to deserialize signature: {e}"))?;
    let is_signature_valid = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .map_err(|e| format!("failed to deserialize public key: {e}"))?
        .verify(message_bytes, &signature)
        .is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

fn sha256(input: &str) -> [u8; 32] {
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
