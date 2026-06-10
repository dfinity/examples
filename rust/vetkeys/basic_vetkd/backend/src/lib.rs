use ic_cdk::api::msg_caller;
use ic_cdk::update;
use ic_cdk_management_canister::{VetKDCurve, VetKDDeriveKeyArgs, VetKDKeyId, VetKDPublicKeyArgs};

#[update]
async fn symmetric_key_verification_key() -> Vec<u8> {
    let request = VetKDPublicKeyArgs {
        canister_id: None,
        context: b"symmetric_key".to_vec(),
        key_id: test_key(),
    };

    ic_cdk_management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed")
        .public_key
}

#[update]
async fn encrypted_symmetric_key_for_caller(transport_public_key: Vec<u8>) -> Vec<u8> {
    let request = VetKDDeriveKeyArgs {
        input: msg_caller().as_slice().to_vec(),
        context: b"symmetric_key".to_vec(),
        key_id: test_key(),
        transport_public_key,
    };

    ic_cdk_management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_derive_key failed")
        .encrypted_key
}

#[update]
async fn ibe_encryption_key() -> Vec<u8> {
    let request = VetKDPublicKeyArgs {
        canister_id: None,
        context: b"ibe_encryption".to_vec(),
        key_id: test_key(),
    };

    ic_cdk_management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed")
        .public_key
}

#[update]
async fn encrypted_ibe_decryption_key_for_caller(transport_public_key: Vec<u8>) -> Vec<u8> {
    let request = VetKDDeriveKeyArgs {
        input: msg_caller().as_slice().to_vec(),
        context: b"ibe_encryption".to_vec(),
        key_id: test_key(),
        transport_public_key,
    };

    ic_cdk_management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_derive_key failed")
        .encrypted_key
}

fn test_key() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381_G2,
        name: "test_key_1".to_string(),
    }
}

ic_cdk::export_candid!();
