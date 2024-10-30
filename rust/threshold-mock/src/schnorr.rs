use crate::ensure_call_is_paid;
use crate::inc_call_count;

use super::ensure_derivation_path_is_valid;
use super::with_rng;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
use ic_cdk::update;

pub type CanisterId = Principal;

#[derive(Clone, CandidType, Deserialize)]
pub struct SchnorrKeyId {
    pub algorithm: SchnorrAlgorithm,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, CandidType, Deserialize)]
pub enum SchnorrAlgorithm {
    #[serde(rename = "bip340secp256k1")]
    Bip340Secp256k1,
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(CandidType, Deserialize)]
pub struct SchnorrPublicKeyArgs {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize)]
pub struct SchnorrPublicKeyResult {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct SignWithSchnorrArgs {
    pub message: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize)]
pub struct SignWithSchnorrResult {
    pub signature: Vec<u8>,
}

/// DISCLAIMER: This canister here provides an *unsafe* example implementation
/// of the Internet Computer's management canister API for demonstration purposes.
/// Because of this, in the following we hard-code a randomly generated master
/// secret key. Do NOT use this in production.
const MASTER_SK_ED25519_HEX: &str =
    "e7183596fae9139be990cdccc7d81341ac628e825c70044e902656a0a3f76b04";

lazy_static::lazy_static! {
    static ref MASTER_SK_ED25519: ic_crypto_ed25519::PrivateKey = ic_crypto_ed25519::PrivateKey::deserialize_raw(
        &hex::decode(MASTER_SK_ED25519_HEX).expect("failed to hex-decode")
    ).expect("failed to deserialize Ed25519 private key");
    static ref MASTER_PK_Ed25519: ic_crypto_ed25519::PublicKey = MASTER_SK_ED25519.public_key();
}

/// DISCLAIMER: This canister here provides an *unsafe* example implementation
/// of the Internet Computer's management canister API for demonstration purposes.
/// Because of this, in the following we hard-code a randomly generated master
/// secret key. Do NOT use this in production.
const MASTER_SK_SECP256K1_HEX: &str =
    "4dbfb616fec0d02573e9219607814c4dc9774b164a5da9e44079db91f4de0c2d";

lazy_static::lazy_static! {
    static ref MASTER_SK_SECP256K1: ic_crypto_secp256k1::PrivateKey = ic_crypto_secp256k1::PrivateKey::deserialize_sec1(
        &hex::decode(MASTER_SK_SECP256K1_HEX).expect("failed to hex-decode")
    ).expect("failed to deserialize secp256k1 private key");
    static ref MASTER_PK_SECP256K1: ic_crypto_secp256k1::PublicKey = MASTER_SK_SECP256K1.public_key();
}

#[update]
async fn schnorr_public_key(args: SchnorrPublicKeyArgs) -> SchnorrPublicKeyResult {
    ensure_derivation_path_is_valid(&args.derivation_path);
    match args.key_id.algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => {
            inc_call_count("schnorr_public_key_bip340Secp256k1".to_string());
            ensure_bip340secp256k1_insecure_test_key_1(&args.key_id);
            let derivation_path = ic_crypto_secp256k1::DerivationPath::from_canister_id_and_path(
                args.canister_id.unwrap_or_else(ic_cdk::caller).as_slice(),
                &args.derivation_path,
            );
            let (public_key, chain_code) = MASTER_PK_SECP256K1.derive_subkey(&derivation_path);
            SchnorrPublicKeyResult {
                public_key: public_key.serialize_sec1(true).to_vec(),
                chain_code: chain_code.to_vec(),
            }
        }
        SchnorrAlgorithm::Ed25519 => {
            inc_call_count("schnorr_public_key_ed25519".to_string());
            ensure_ed25519_insecure_test_key_1(&args.key_id);
            let derivation_path = ic_crypto_ed25519::DerivationPath::from_canister_id_and_path(
                args.canister_id.unwrap_or_else(ic_cdk::caller).as_slice(),
                &args.derivation_path,
            );
            let (public_key, chain_code) = MASTER_PK_Ed25519.derive_subkey(&derivation_path);
            SchnorrPublicKeyResult {
                public_key: public_key.serialize_raw().to_vec(),
                chain_code: chain_code.to_vec(),
            }
        }
    }
}

#[update]
async fn sign_with_schnorr(args: SignWithSchnorrArgs) -> SignWithSchnorrResult {
    ensure_call_is_paid(0);
    ensure_derivation_path_is_valid(&args.derivation_path);
    match args.key_id.algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => {
            inc_call_count("sign_with_schnorr_bip340Secp256k1".to_string());
            ensure_bip340secp256k1_insecure_test_key_1(&args.key_id);
            let derivation_path = ic_crypto_secp256k1::DerivationPath::from_canister_id_and_path(
                ic_cdk::caller().as_slice(),
                &args.derivation_path,
            );
            let (derived_private_key, _chain_code) =
                MASTER_SK_SECP256K1.derive_subkey(&derivation_path);
            let signature =
                with_rng(|rng| derived_private_key.sign_message_with_bip340(&args.message, rng))
                    .await;
            SignWithSchnorrResult {
                signature: signature.to_vec(),
            }
        }
        SchnorrAlgorithm::Ed25519 => {
            inc_call_count("sign_with_schnorr_ed25519".to_string());
            ensure_ed25519_insecure_test_key_1(&args.key_id);
            let derivation_path = ic_crypto_ed25519::DerivationPath::from_canister_id_and_path(
                ic_cdk::caller().as_slice(),
                &args.derivation_path,
            );
            let (derived_private_key, _chain_code) =
                MASTER_SK_ED25519.derive_subkey(&derivation_path);
            let signature = derived_private_key.sign_message(&args.message);
            SignWithSchnorrResult {
                signature: signature.to_vec(),
            }
        }
    }
}

fn ensure_ed25519_insecure_test_key_1(key_id: &SchnorrKeyId) {
    if key_id.algorithm != SchnorrAlgorithm::Ed25519 {
        ic_cdk::trap("unsupported key ID algorithm");
    }
    if key_id.name.as_str() != "insecure_test_key_1" {
        ic_cdk::trap("unsupported key ID name");
    }
}

fn ensure_bip340secp256k1_insecure_test_key_1(key_id: &SchnorrKeyId) {
    if key_id.algorithm != SchnorrAlgorithm::Bip340Secp256k1 {
        ic_cdk::trap("unsupported key ID algorithm");
    }
    if key_id.name.as_str() != "insecure_test_key_1" {
        ic_cdk::trap("unsupported key ID name");
    }
}
