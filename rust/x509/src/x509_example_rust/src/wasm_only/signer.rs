use std::{convert::TryFrom, ops::Deref};

use candid::Principal;
use elliptic_curve::sec1::ToEncodedPoint;
use ic_cdk::api::management_canister::ecdsa as cdk_ecdsa;
use sha2::Digest;
use spki::{AlgorithmIdentifierOwned, DynSignatureAlgorithmIdentifier};

use super::{
    derivation_path, root_ca_public_key_bytes, ManagementCanisterSignatureReply,
    ManagementCanisterSignatureRequest, SchnorrKeyId, CA_KEY_INFORMATION,
};

pub trait Sign {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, String>;
}

pub struct Ed25519Signer {
    key_id: SchnorrKeyId,
    public_key: ed25519::pkcs8::PublicKeyBytes,
}

impl Ed25519Signer {
    pub async fn new() -> Result<Self, String> {
        let public_key_raw = <[u8; 32]>::try_from(root_ca_public_key_bytes().await?.as_slice())
            .map_err(|e| format!("public key has wrong length: {e:?}"))?;
        let public_key = ed25519::pkcs8::PublicKeyBytes(public_key_raw);
        Ok(Self {
            key_id: CA_KEY_INFORMATION
                .with(|value| SchnorrKeyId::try_from(value.borrow().deref()))?,
            public_key,
        })
    }
}

impl Sign for Ed25519Signer {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, String> {
        let internal_request = ManagementCanisterSignatureRequest {
            message: msg.to_vec(),
            derivation_path: derivation_path(), // empty because there is only one root certificate for everyone in this example
            key_id: self.key_id.clone(),
        };

        let (internal_reply,): (ManagementCanisterSignatureReply,) =
            ic_cdk::api::call::call_with_payment(
                Principal::management_canister(),
                "sign_with_schnorr",
                (internal_request,),
                26_153_846_153,
            )
            .await
            .map_err(|e| format!("sign_with_schnorr failed {e:?}"))?;

        Ok(internal_reply.signature)
    }
}

impl AsRef<ed25519::pkcs8::PublicKeyBytes> for Ed25519Signer {
    fn as_ref(&self) -> &ed25519::pkcs8::PublicKeyBytes {
        &self.public_key
    }
}

impl signature::KeypairRef for Ed25519Signer {
    type VerifyingKey = ed25519::pkcs8::PublicKeyBytes;
}

impl DynSignatureAlgorithmIdentifier for Ed25519Signer {
    fn signature_algorithm_identifier(&self) -> spki::Result<AlgorithmIdentifierOwned> {
        Ok(AlgorithmIdentifierOwned {
            oid: ed25519::pkcs8::ALGORITHM_OID.clone(),
            parameters: None,
        })
    }
}

pub struct EcdsaSecp256k1Signer {
    key_id: cdk_ecdsa::EcdsaKeyId,
    public_key: k256::ecdsa::VerifyingKey,
}

impl EcdsaSecp256k1Signer {
    pub async fn new() -> Result<Self, String> {
        let public_key = k256::ecdsa::VerifyingKey::from_encoded_point(
            &k256::PublicKey::from_sec1_bytes(&root_ca_public_key_bytes().await?.as_slice())
                .map_err(|e| format!("malformed public key: {e:?}"))?
                .to_encoded_point(false),
        )
        .unwrap();
        Ok(Self {
            key_id: CA_KEY_INFORMATION
                .with(|value| cdk_ecdsa::EcdsaKeyId::try_from(value.borrow().deref()))?,
            public_key,
        })
    }
}

impl Sign for EcdsaSecp256k1Signer {
    async fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, String> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(msg);

        let args = cdk_ecdsa::SignWithEcdsaArgument {
            message_hash: hasher.finalize().to_vec(),
            derivation_path: derivation_path(),
            key_id: self.key_id.clone(),
        };

        let (internal_reply,): (cdk_ecdsa::SignWithEcdsaResponse,) =
            cdk_ecdsa::sign_with_ecdsa(args)
                .await
                .map_err(|e| format!("sign_with_schnorr failed {e:?}"))?;

        Ok(
            k256::ecdsa::Signature::from_slice(internal_reply.signature.as_slice())
                .map_err(|e| format!("malformed signature: {e:?}"))?
                .to_der()
                .to_bytes()
                .to_vec(),
        )
    }
}

impl AsRef<k256::ecdsa::VerifyingKey> for EcdsaSecp256k1Signer {
    fn as_ref(&self) -> &k256::ecdsa::VerifyingKey {
        &self.public_key
    }
}

impl signature::KeypairRef for EcdsaSecp256k1Signer {
    type VerifyingKey = k256::ecdsa::VerifyingKey;
}

impl DynSignatureAlgorithmIdentifier for EcdsaSecp256k1Signer {
    fn signature_algorithm_identifier(&self) -> spki::Result<AlgorithmIdentifierOwned> {
        Ok(AlgorithmIdentifierOwned {
            oid: ecdsa::ECDSA_SHA256_OID,
            parameters: None,
        })
    }
}
