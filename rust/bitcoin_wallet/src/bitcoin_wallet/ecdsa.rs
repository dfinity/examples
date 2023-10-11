use crate::{
    canister_common::SIGN_WITH_ECDSA_COST_CYCLES,
    types::{
        ECDSAPublicKey, ECDSAPublicKeyReply, EcdsaCurve, EcdsaKeyId, EcdsaPubKey,
        ManagementCanisterReject, SignWithECDSA, SignWithECDSAReply,
    },
};
use bitcoin::Network;
use candid::Principal;
use ic_cdk::{api::call::call_with_payment, call};

/// Returns the key name associated with a given Bitcoin network.
pub(crate) fn get_key_name_from_network(network: Network) -> String {
    String::from(match network {
        // A special test key with dfx is used for local development.
        Network::Regtest => "dfx_test_key",
        // A test ECDSA key is used on the IC.
        _ => "test_key_1",
    })
}

/// Returns the Bitcoin ECDSA public key of this canister.
pub(crate) async fn get_btc_ecdsa_public_key(
    key_name: &String,
) -> Result<EcdsaPubKey, ManagementCanisterReject> {
    let ecdsa_public_key_reply = ecdsa_public_key(key_name, &[]).await?;
    Ok(EcdsaPubKey {
        public_key: ecdsa_public_key_reply.public_key,
        chain_code: ecdsa_public_key_reply.chain_code,
        derivation_path: vec![],
    })
}

/// Returns the ECDSA public key of this canister at the given derivation path.
pub(crate) async fn ecdsa_public_key(
    key_name: &String,
    derivation_path: &[Vec<u8>],
) -> Result<ECDSAPublicKeyReply, ManagementCanisterReject> {
    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.
    let res: Result<(ECDSAPublicKeyReply,), _> = call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id: None,
            derivation_path: derivation_path.to_vec(),
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name.to_string(),
            },
        },),
    )
    .await;

    match res {
        // Return the ECDSA public key to the caller.
        Ok(data) => Ok(data.0),

        // The call to `ecdsa_public_key` was rejected for a given reason (e.g., not enough cycles were attached to the call).
        Err((rejection_code, message)) => Err(ManagementCanisterReject(rejection_code, message)),
    }
}

/// Returns the signature of the given `message_hash` associated with the ECDSA public key
/// of this canister at the given derivation path.
pub(crate) async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Result<Vec<u8>, ManagementCanisterReject> {
    let res: Result<(SignWithECDSAReply,), _> = call_with_payment(
        Principal::management_canister(),
        "sign_with_ecdsa",
        (SignWithECDSA {
            message_hash,
            derivation_path,
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: key_name,
            },
        },),
        SIGN_WITH_ECDSA_COST_CYCLES,
    )
    .await;

    match res {
        // Return the signature to the caller.
        Ok(data) => Ok(data.0.signature),

        // The call to `sign_with_ecdsa` was rejected for a given reason (e.g., not enough cycles were attached to the call).
        Err((rejection_code, message)) => Err(ManagementCanisterReject(rejection_code, message)),
    }
}
