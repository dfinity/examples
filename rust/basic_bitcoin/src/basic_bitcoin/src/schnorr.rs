use ic_cdk::management_canister::{
    self, SchnorrAlgorithm, SchnorrAux, SchnorrKeyId, SchnorrPublicKeyArgs, SignWithSchnorrArgs,
};

/// Returns the Schnorr public key of this canister at the given derivation path.
pub async fn schnorr_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    management_canister::schnorr_public_key(&SchnorrPublicKeyArgs {
        canister_id: None,
        derivation_path,
        key_id: SchnorrKeyId {
            name: key_name,
            algorithm: SchnorrAlgorithm::Bip340secp256k1,
        },
    })
    .await
    .unwrap()
    .public_key
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
