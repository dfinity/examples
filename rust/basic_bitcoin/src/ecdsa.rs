use crate::BitcoinContext;
use bitcoin::secp256k1::ecdsa::Signature;
use ic_cdk::management_canister::{
    self, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgs, SignWithEcdsaArgs,
};
use std::{cell::RefCell, collections::HashMap};

type DerivationPath = Vec<Vec<u8>>;
type EcdsaKey = Vec<u8>;

// In-memory cache for ECDSA public keys. Note: this cache is not persistent across smart contract upgrades.
thread_local! {
    static ECDSA_KEY_CACHE: RefCell<HashMap<DerivationPath, EcdsaKey>> = RefCell::new(HashMap::new());
}

/// Retrieves the ECDSA public key for the given derivation path from the ECDSA API.
///
/// This function checks the local in-memory cache first. If no cached key exists,
/// it queries the ECDSA API for the public key at the given derivation path
/// and stores the result in the cache.
pub async fn get_ecdsa_public_key(ctx: &BitcoinContext, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Check in-memory cache first.
    if let Some(key) = ECDSA_KEY_CACHE.with_borrow(|map| map.get(&derivation_path).cloned()) {
        return key;
    }

    // Request the ECDSA public key from the ECDSA API.
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

    // Store it in the in-memory cache for future reuse.
    ECDSA_KEY_CACHE.with_borrow_mut(|map| {
        map.insert(derivation_path, public_key.clone());
    });

    public_key
}

/// Signs a 32-byte message hash using the ECDSA key derived from the given path.
///
/// This function uses the ICP ECDSA signing API to produce a compact, 64-byte signature.
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

/// Returns a mock ECDSA signature used solely for **transaction size estimation**.
///
/// This function returns a fixed-size, syntactically valid but cryptographically invalid
/// ECDSA signature. It is **not** suitable for use in real transactions
/// but is useful when constructing a Bitcoin transaction to estimate its weight or fee.
///
/// # Safety
/// Do not broadcast transactions signed with this signature.
pub async fn mock_sign_with_ecdsa(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _signing_data: Vec<u8>,
) -> Signature {
    let r_s = [1u8; 64];
    Signature::from_compact(&r_s).unwrap()
}
