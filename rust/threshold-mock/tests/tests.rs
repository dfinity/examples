use assert_matches::assert_matches;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use ic_vetkd_utils::TransportSecretKey;
use pocket_ic::{PocketIc, WasmResult};

use ic_cdk::api::management_canister::ecdsa::EcdsaCurve;
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyArgument;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaResponse;
use chainkey_testing_canister::schnorr::SchnorrAlgorithm;
use chainkey_testing_canister::schnorr::SchnorrKeyId;
use chainkey_testing_canister::schnorr::SchnorrPublicKeyArgs;
use chainkey_testing_canister::schnorr::SchnorrPublicKeyResult;
use chainkey_testing_canister::schnorr::SignWithSchnorrArgs;
use chainkey_testing_canister::schnorr::SignWithSchnorrResult;
use chainkey_testing_canister::vetkd::VetKDCurve;
use chainkey_testing_canister::vetkd::VetKDEncryptedKeyReply;
use chainkey_testing_canister::vetkd::VetKDEncryptedKeyRequest;
use chainkey_testing_canister::vetkd::VetKDKeyId;
use chainkey_testing_canister::vetkd::VetKDPublicKeyReply;
use chainkey_testing_canister::vetkd::VetKDPublicKeyRequest;

pub const CANISTER_WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/chainkey_testing_canister.wasm");

#[test]
fn should_verify_ecdsa_signature() {
    let canister = CanisterSetup::default();

    let derivation_path = vec!["test-derivation-path".as_bytes().to_vec()];
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "insecure_mock_key_1".to_string(),
    };
    let message_hash = b"12345678901234567890123456789012".to_vec();

    let public_key_raw = canister
        .ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: derivation_path.clone(),
            key_id: key_id.clone(),
        })
        .public_key;

    let signature_raw = canister
        .sign_with_ecdsa(SignWithEcdsaArgument {
            message_hash: message_hash.clone(),
            derivation_path,
            key_id,
        })
        .signature;

    let signature = k256::ecdsa::Signature::try_from(signature_raw.as_slice())
        .expect("failed to deserialize signature");
    let verifying_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_raw)
        .expect("failed to sec1 deserialize public key");

    use k256::ecdsa::signature::hazmat::PrehashVerifier;
    assert_matches!(
        verifying_key.verify_prehash(&message_hash, &signature),
        Ok(())
    );
}

#[test]
fn should_verify_schnorr_bip340_secp256k1_signature() {
    let canister = CanisterSetup::default();

    let derivation_path = vec!["test-derivation-path".as_bytes().to_vec()];
    let key_id = SchnorrKeyId {
        algorithm: SchnorrAlgorithm::Bip340Secp256k1,
        name: "insecure_mock_key_1".to_string(),
    };
    let message = b"test-message".to_vec();

    let public_key_raw = canister
        .schnorr_public_key(SchnorrPublicKeyArgs {
            canister_id: None,
            derivation_path: derivation_path.clone(),
            key_id: key_id.clone(),
        })
        .public_key;

    let signature_raw = canister
        .sign_with_schnorr(SignWithSchnorrArgs {
            message: message.clone(),
            derivation_path,
            key_id,
        })
        .signature;

    let signature = k256::schnorr::Signature::try_from(signature_raw.as_slice())
        .expect("failed to deserialize signature");
    let verifying_key = k256::schnorr::VerifyingKey::from_bytes(&public_key_raw[1..])
        .expect("failed to sec1 deserialize public key");

    assert_matches!(verifying_key.verify_raw(&message, &signature), Ok(()));
}

#[test]
fn should_verify_schnorr_ed25519_signature() {
    let canister = CanisterSetup::default();

    let derivation_path = vec!["test-derivation-path".as_bytes().to_vec()];
    let key_id = SchnorrKeyId {
        algorithm: SchnorrAlgorithm::Ed25519,
        name: "insecure_mock_key_1".to_string(),
    };
    let message = b"test-message".to_vec();

    let public_key_raw = canister
        .schnorr_public_key(SchnorrPublicKeyArgs {
            canister_id: None,
            derivation_path: derivation_path.clone(),
            key_id: key_id.clone(),
        })
        .public_key;

    let signature_raw = canister
        .sign_with_schnorr(SignWithSchnorrArgs {
            message: message.clone(),
            derivation_path,
            key_id,
        })
        .signature;

    let signature = ed25519_dalek::Signature::try_from(signature_raw.as_slice())
        .expect("failed to deserialize signature");
    let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(
        &<[u8; 32]>::try_from(public_key_raw).expect("ed25519 public key should be 32 bytes"),
    )
    .expect("failed to sec1 deserialize public key");

    use ed25519_dalek::Verifier;
    assert_matches!(verifying_key.verify(&message, &signature), Ok(()));
}

#[test]
fn should_consistently_derive_vetkey() {
    let canister = CanisterSetup::default();

    let derivation_path = vec!["test-derivation-path".as_bytes().to_vec()];
    let key_id = VetKDKeyId {
        curve: VetKDCurve::Bls12_381,
        name: "insecure_mock_key_1".to_string(),
    };
    let derivation_id = b"test-derivation-id".to_vec();

    let public_key_bytes = canister
        .vetkd_public_key(VetKDPublicKeyRequest {
            canister_id: None,
            derivation_path: derivation_path.clone(),
            key_id: key_id.clone(),
        })
        .public_key;

    let tsk_1 = TransportSecretKey::from_seed([101; 32].to_vec())
        .expect("failed to create transport secret key");
    let encrypted_key_1 = canister
        .vetkd_encrypted_key(VetKDEncryptedKeyRequest {
            public_key_derivation_path: derivation_path.clone(),
            derivation_id: derivation_id.clone(),
            encryption_public_key: tsk_1.public_key(),
            key_id: key_id.clone(),
        })
        .encrypted_key;
    let decrypted_key_1 = tsk_1
        .decrypt(&encrypted_key_1, &public_key_bytes, &derivation_id)
        .expect("failed to decrypted vetKey");

    let tsk_2 = TransportSecretKey::from_seed([102; 32].to_vec())
        .expect("failed to create transport secret key");
    let encrypted_key_2 = canister
        .vetkd_encrypted_key(VetKDEncryptedKeyRequest {
            public_key_derivation_path: derivation_path,
            derivation_id: derivation_id.clone(),
            encryption_public_key: tsk_2.public_key(),
            key_id,
        })
        .encrypted_key;
    let decrypted_key_2 = tsk_2
        .decrypt(&encrypted_key_2, &public_key_bytes, &derivation_id)
        .expect("failed to decrypted vetKey");

    assert_eq!(decrypted_key_1, decrypted_key_2);
}

pub struct CanisterSetup {
    env: PocketIc,
    canister_id: CanisterId,
}

impl CanisterSetup {
    pub fn new() -> Self {
        let env = PocketIc::new();
        let canister_id = env.create_canister();
        env.add_cycles(canister_id, u128::MAX);
        env.install_canister(canister_id, CANISTER_WASM.to_vec(), vec![], None);
        Self { env, canister_id }
    }

    pub fn ecdsa_public_key(&self, args: EcdsaPublicKeyArgument) -> EcdsaPublicKeyResponse {
        let method = "ecdsa_public_key";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode ecdsa_public_key args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, EcdsaPublicKeyResponse).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }

    pub fn sign_with_ecdsa(&self, args: SignWithEcdsaArgument) -> SignWithEcdsaResponse {
        let method = "sign_with_ecdsa";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, SignWithEcdsaResponse).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }

    pub fn schnorr_public_key(&self, args: SchnorrPublicKeyArgs) -> SchnorrPublicKeyResult {
        let method = "schnorr_public_key";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, SchnorrPublicKeyResult).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }

    pub fn sign_with_schnorr(&self, args: SignWithSchnorrArgs) -> SignWithSchnorrResult {
        let method = "sign_with_schnorr";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, SignWithSchnorrResult).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }

    pub fn vetkd_public_key(&self, args: VetKDPublicKeyRequest) -> VetKDPublicKeyReply {
        let method = "vetkd_public_key";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, VetKDPublicKeyReply).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }

    pub fn vetkd_encrypted_key(&self, args: VetKDEncryptedKeyRequest) -> VetKDEncryptedKeyReply {
        let method = "vetkd_encrypted_key";
        let result = self.env.update_call(
            self.canister_id,
            Principal::anonymous(),
            method,
            Encode!(&args).expect("failed to encode args"),
        );
        match result {
            Ok(WasmResult::Reply(bytes)) => {
                Decode!(&bytes, VetKDEncryptedKeyReply).expect("failed to decode {method} result")
            }
            Ok(WasmResult::Reject(error)) => {
                panic!("canister rejected call to {method}: {error}")
            }
            Err(user_error) => panic!("{method} user error: {user_error}"),
        }
    }
}

impl Default for CanisterSetup {
    fn default() -> Self {
        Self::new()
    }
}
