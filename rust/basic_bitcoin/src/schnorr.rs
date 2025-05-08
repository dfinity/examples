use std::{cell::RefCell, collections::HashMap};

use crate::BitcoinContext;
use ic_cdk::management_canister::{
    self, SchnorrAlgorithm, SchnorrAux, SchnorrKeyId, SchnorrPublicKeyArgs, SignWithSchnorrArgs,
};

type DerivationPath = Vec<Vec<u8>>;
type SchnorrKey = Vec<u8>;

// Cache for Schnorr public keys. Cache does not persist across canister upgrades.
thread_local! {
    static SCHNORR_KEY_CACHE: RefCell<HashMap<DerivationPath, SchnorrKey>> = RefCell::new(HashMap::new());
}

/// Returns the Schnorr public key of this canister at the given derivation path.
pub async fn get_schnorr_public_key(
    ctx: &BitcoinContext,
    derivation_path: Vec<Vec<u8>>,
) -> Vec<u8> {
    // Retrieve and return already stored public key
    if let Some(key) = SCHNORR_KEY_CACHE.with_borrow(|map| map.get(&derivation_path).cloned()) {
        return key;
    }

    let public_key = management_canister::schnorr_public_key(&SchnorrPublicKeyArgs {
        canister_id: None,
        derivation_path: derivation_path.clone(),
        key_id: SchnorrKeyId {
            name: ctx.key_name.to_string(),
            algorithm: SchnorrAlgorithm::Bip340secp256k1,
        },
    })
    .await
    .unwrap()
    .public_key;

    // Cache the public key
    SCHNORR_KEY_CACHE.with_borrow_mut(|map| {
        map.insert(derivation_path, public_key.clone());
    });

    public_key
}

/// Returns the signature for `message` by a private and *distributed* private
/// key derived from `key_name`, `derivation_path`, and the optional
/// [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)
/// `merkle_root_hash`.
pub async fn sign_with_schnorr(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    merkle_root_hash: Option<Vec<u8>>,
    message: Vec<u8>,
) -> Vec<u8> {
    let aux = merkle_root_hash.map(|bytes| {
        SchnorrAux::Bip341(management_canister::Bip341 {
            merkle_root_hash: bytes,
        })
    });

    management_canister::sign_with_schnorr(&SignWithSchnorrArgs {
        message,
        derivation_path,
        key_id: SchnorrKeyId {
            name: key_name,
            algorithm: SchnorrAlgorithm::Bip340secp256k1,
        },
        aux,
    })
    .await
    .unwrap()
    .signature
}

// A mock for rubber-stamping signatures.
pub async fn mock_sign_with_schnorr(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _merkle_root_hash: Option<Vec<u8>>,
    _message_hash: Vec<u8>,
) -> Vec<u8> {
    vec![255; 64]
}
