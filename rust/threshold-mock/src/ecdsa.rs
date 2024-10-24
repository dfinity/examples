use super::ensure_derivation_path_is_valid;
use ic_cdk::api::management_canister::ecdsa::EcdsaCurve;
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyArgument;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaResponse;
use ic_cdk::update;

/// DISCLAIMER: This canister here provides an *unsafe* example implementation
/// of the Internet Computer's management canister API for demonstration purposes.
/// Because of this, in the following we hard-code a randomly generated master
/// secret key. Do NOT use this in production.
const MASTER_SK_SECP256K1_HEX: &str =
    "bcd854ea5112b314b5d306442376b57a6e6f2ffc34c2d0f6918823b0f7c0bfac";

lazy_static::lazy_static! {
    static ref MASTER_SK_SECP256K1: ic_crypto_secp256k1::PrivateKey = ic_crypto_secp256k1::PrivateKey::deserialize_sec1(
        &hex::decode(MASTER_SK_SECP256K1_HEX).expect("failed to hex-decode")
    ).expect("failed to deserialize secp256k1 private key");
    static ref MASTER_PK_SECP256K1: ic_crypto_secp256k1::PublicKey = MASTER_SK_SECP256K1.public_key();
}

#[update]
async fn ecdsa_public_key(args: EcdsaPublicKeyArgument) -> EcdsaPublicKeyResponse {
    ensure_secp256k1_insecure_mock_key_1(&args.key_id);
    ensure_derivation_path_is_valid(&args.derivation_path);
    let derivation_path = ic_crypto_secp256k1::DerivationPath::from_canister_id_and_path(
        args.canister_id.unwrap_or_else(ic_cdk::caller).as_slice(),
        &args.derivation_path,
    );
    let (public_key, chain_code) = MASTER_PK_SECP256K1.derive_subkey(&derivation_path);
    EcdsaPublicKeyResponse {
        public_key: public_key.serialize_sec1(true).to_vec(),
        chain_code: chain_code.to_vec(),
    }
}

#[update]
async fn sign_with_ecdsa(args: SignWithEcdsaArgument) -> SignWithEcdsaResponse {
    ensure_secp256k1_insecure_mock_key_1(&args.key_id);
    ensure_derivation_path_is_valid(&args.derivation_path);
    if args.message_hash.len() != 32 {
        ic_cdk::trap("message hash must be 32 bytes");
    }
    let derivation_path = ic_crypto_secp256k1::DerivationPath::from_canister_id_and_path(
        ic_cdk::caller().as_slice(),
        &args.derivation_path,
    );
    let (derived_private_key, _chain_code) = MASTER_SK_SECP256K1.derive_subkey(&derivation_path);
    let signature = derived_private_key.sign_digest_with_ecdsa(&args.message_hash);
    SignWithEcdsaResponse {
        signature: signature.to_vec(),
    }
}

fn ensure_secp256k1_insecure_mock_key_1(key_id: &EcdsaKeyId) {
    if key_id.curve != EcdsaCurve::Secp256k1 {
        ic_cdk::trap("unsupported key ID curve");
    }
    if key_id.name.as_str() != "insecure_mock_key_1" {
        ic_cdk::trap("unsupported key ID name");
    }
}
