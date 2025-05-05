use crate::BitcoinContext;
use bitcoin::secp256k1::ecdsa::Signature;
use ic_cdk::management_canister::{
    self, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs, SignWithEcdsaArgs,
};
use std::{cell::RefCell, collections::HashMap};

type DerivationPath = Vec<Vec<u8>>;
type EcdsaKey = Vec<u8>;

// Cache for ECDSA public keys. Cache does not persist across canister upgrades.
thread_local! {
    static ECDSA_KEY_CACHE: RefCell<HashMap<DerivationPath, EcdsaKey>> = RefCell::new(HashMap::new());
}

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn get_ecdsa_public_key(ctx: &BitcoinContext, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Retrieve and return already stored public key
    if let Some(key) = ECDSA_KEY_CACHE.with_borrow(|map| map.get(&derivation_path).cloned()) {
        return key;
    }

    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let public_key = management_canister::ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id: None,
        derivation_path: derivation_path.clone(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: ctx.key_name.to_string(),
        },
    })
    .await
    .unwrap()
    .public_key;

    // Cache the public key
    ECDSA_KEY_CACHE.with_borrow_mut(|map| {
        map.insert(derivation_path, public_key.clone());
    });

    public_key
}

pub async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Signature {
    let signature = management_canister::sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash,
        derivation_path,
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: key_name,
        },
    })
    .await
    .unwrap()
    .signature;
    Signature::from_compact(&signature).unwrap()
}

pub async fn mock_sign_with_ecdsa(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _signing_data: Vec<u8>,
) -> Signature {
    let r_s = [1u8; 64];
    Signature::from_compact(&r_s).unwrap()
}
