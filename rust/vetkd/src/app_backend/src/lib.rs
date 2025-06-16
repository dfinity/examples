use candid::Principal;
use ic_cdk::api::msg_caller;
use ic_cdk::management_canister::{VetKDCurve, VetKDDeriveKeyArgs, VetKDKeyId, VetKDPublicKeyArgs};
use ic_cdk::update;

#[update]
async fn symmetric_key_verification_key() -> String {
    let request = VetKDPublicKeyArgs {
        canister_id: None,
        context: b"symmetric_key".to_vec(),
        key_id: bls12_381_g2_dfx_test_key(),
    };

    let response = ic_cdk::management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_symmetric_key_for_caller(transport_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_symmetric_key_for_caller");

    let request = VetKDDeriveKeyArgs {
        input: msg_caller().as_slice().to_vec(),
        context: b"symmetric_key".to_vec(),
        key_id: bls12_381_g2_dfx_test_key(),
        transport_public_key,
    };

    let response = ic_cdk::management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_derive_key failed");

    hex::encode(response.encrypted_key)
}

#[update]
async fn ibe_encryption_key() -> String {
    let request = VetKDPublicKeyArgs {
        canister_id: None,
        context: b"ibe_encryption".to_vec(),
        key_id: bls12_381_g2_dfx_test_key(),
    };

    let response = ic_cdk::management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_ibe_decryption_key_for_caller(transport_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_ibe_decryption_key_for_caller");

    let request = VetKDDeriveKeyArgs {
        input: msg_caller().as_slice().to_vec(),
        context: b"ibe_encryption".to_vec(),
        key_id: bls12_381_g2_dfx_test_key(),
        transport_public_key,
    };

    let response = ic_cdk::management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_derive_key failed");

    hex::encode(response.encrypted_key)
}

fn bls12_381_g2_dfx_test_key() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381_G2,
        name: "dfx_test_key".to_string(),
    }
}

fn debug_println_caller(method_name: &str) {
    ic_cdk::println!(
        "{}: caller: {} (isAnonymous: {})",
        method_name,
        ic_cdk::api::msg_caller().to_text(),
        ic_cdk::api::msg_caller() == Principal::anonymous()
    );
}
