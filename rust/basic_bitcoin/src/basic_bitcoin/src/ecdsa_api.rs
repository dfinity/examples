use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    SignWithEcdsaArgument,
};
use std::cell::RefCell;
use std::collections::HashMap;

// stores the ecdsa to maintain state across different calls to the canister (not across updates)
thread_local! {
    /* flexible */ static ECDSA: RefCell<Option<HashMap<Vec<Vec<u8>> /*derivation path*/, Vec<u8> /*public key*/>>> = RefCell::default();
}

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn get_ecdsa_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Retrieve already stored public key
    if let Some(key) = ECDSA.with(|ecdsa| {
        ecdsa.borrow().as_ref().and_then(|map| map.get(&derivation_path).cloned())
    }) {
        return key;
    }
    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let canister_id = None;
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: key_name,
    };

    let res = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id,
        derivation_path: derivation_path.clone(),
        key_id,
    })
    .await;

    let public_key = res.unwrap().0.public_key;

    // Cache the public key
    ECDSA.with(|ecdsa| {
        let mut map = ecdsa.borrow_mut();
        if map.is_none() {
            *map = Some(HashMap::new());
        }
        map.as_mut().unwrap().insert(derivation_path, public_key.clone());
    });

    public_key
}

pub async fn get_ecdsa_signature(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Vec<u8> {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: key_name,
    };

    let res = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash,
        derivation_path,
        key_id,
    })
    .await;

    res.unwrap().0.signature
}
