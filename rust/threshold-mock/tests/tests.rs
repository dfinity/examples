use assert_matches::assert_matches;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use pocket_ic::{PocketIc, WasmResult};

use ic_cdk::api::management_canister::ecdsa::EcdsaCurve;
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyArgument;
use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaArgument;
use ic_cdk::api::management_canister::ecdsa::SignWithEcdsaResponse;

pub const CANISTER_WASM: &[u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/threshold_mock.wasm");

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

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_raw.as_slice())
        .expect("failed to deserialize signature");
    let verifying_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key_raw)
        .expect("failed to sec1 deserialize public key");

    assert_matches!(verifying_key.verify(&message_hash, &signature), Ok(()));
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
            Encode!(&args).expect("failed to encode ecdsa_public_key args"),
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
}

impl Default for CanisterSetup {
    fn default() -> Self {
        Self::new()
    }
}
