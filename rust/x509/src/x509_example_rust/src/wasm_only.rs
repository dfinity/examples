use crate::SchnorrKeyName;

use super::{PublicKeyReply, SchnorrAlgorithm};
use candid::{CandidType, Principal};
use der::{asn1::BitString, pem::LineEnding, DecodePem, Encode, EncodePem};
use ic_cdk::export_candid;
use ic_cdk::{api::time, init, update};
use serde::{Deserialize, Serialize};
use spki::{AlgorithmIdentifier, AlgorithmIdentifierOwned, DynSignatureAlgorithmIdentifier};
use std::{cell::RefCell, convert::TryFrom, error::Error, str::FromStr, time::Duration};
use x509_cert::{
    builder::{Builder, CertificateBuilder, Profile},
    name::Name,
    request::CertReq,
    serial_number::SerialNumber,
    spki::SubjectPublicKeyInfoOwned,
    time::{Time, Validity},
};

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ManagementCanisterSchnorrPublicKeyRequest {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ManagementCanisterSchnorrPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct SchnorrKeyId {
    pub algorithm: SchnorrAlgorithm,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug)]
struct ManagementCanisterSignatureRequest {
    pub message: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: SchnorrKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ManagementCanisterSignatureReply {
    pub signature: Vec<u8>,
}

thread_local! {
    static STATE: RefCell<SchnorrKeyName> = RefCell::new(SchnorrKeyName::DfxTestKey);
}

#[init]
fn init(key_name: SchnorrKeyName) {
    STATE.with(|state| {
        *state.borrow_mut() = key_name;
    });
}

fn key_id() -> SchnorrKeyId {
    STATE.with(|state| {
        let name = String::from(<&'static str>::from(*state.borrow()));
        SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name,
        }
    })
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct X509CertificateString {
    x509_certificate_string: String,
}

struct Signer {
    key_id: SchnorrKeyId,
    public_key: ed25519::pkcs8::PublicKeyBytes,
}
impl Signer {
    pub async fn new() -> Result<Self, String> {
        let public_key_raw = <[u8; 32]>::try_from(root_ca_public_key_bytes().await?.as_slice())
            .map_err(|e| format!("public key has wrong length: {e:?}"))?;
        let public_key = ed25519::pkcs8::PublicKeyBytes(public_key_raw);
        Ok(Self {
            key_id: key_id(),
            public_key,
        })
    }

    pub async fn sign(&self, msg: &[u8]) -> Result<ed25519::Signature, String> {
        let internal_request = ManagementCanisterSignatureRequest {
            message: msg.to_vec(),
            derivation_path: vec![], // empty because there is only one root certificate for everyone in this example
            key_id: self.key_id.clone(),
        };

        let (internal_reply,): (ManagementCanisterSignatureReply,) =
            ic_cdk::api::call::call_with_payment(
                Principal::management_canister(),
                "sign_with_schnorr",
                (internal_request,),
                25_000_000_000,
            )
            .await
            .map_err(|e| format!("sign_with_schnorr failed {e:?}"))?;

        Ok(ed25519::Signature::from_bytes(
            &<[u8; 64]>::try_from(internal_reply.signature.as_slice())
                .map_err(|e| format!("signature has wrong length: {e:?}"))?,
        ))
    }
}

impl AsRef<ed25519::pkcs8::PublicKeyBytes> for Signer {
    fn as_ref(&self) -> &ed25519::pkcs8::PublicKeyBytes {
        &self.public_key
    }
}

impl signature::KeypairRef for Signer {
    type VerifyingKey = ed25519::pkcs8::PublicKeyBytes;
}

impl DynSignatureAlgorithmIdentifier for Signer {
    fn signature_algorithm_identifier(&self) -> spki::Result<AlgorithmIdentifierOwned> {
        Ok(AlgorithmIdentifierOwned {
            oid: ed25519::pkcs8::ALGORITHM_OID.clone(),
            parameters: None,
        })
    }
}

#[update]
async fn root_ca_certificate() -> Result<X509CertificateString, String> {
    let profile = Profile::Root;
    let serial_number = SerialNumber::from(0u32);
    let validity = validity();

    let subject = Name::from_str(
        "CN=Web3 certification authority corporation,O=Web3 ceritifcation authority Inc,C=US",
    )
    .unwrap();

    let algorithm = AlgorithmIdentifier::<der::Any> {
        oid: ed25519::pkcs8::ALGORITHM_OID,
        parameters: None,
    };

    let subject_public_key = der::asn1::BitString::new(0, root_ca_public_key_bytes().await?)
        .map_err(|e| format!("source: {:?}", e.source()))?;

    let pub_key = SubjectPublicKeyInfoOwned {
        algorithm,
        subject_public_key,
    };

    let signer = Signer::new().await?;

    let mut builder =
        CertificateBuilder::new(profile, serial_number, validity, subject, pub_key, &signer)
            .expect("Create certificate");

    let blob = builder
        .finalize()
        .map_err(|e| format!("failed to finalize certificate builder: {e:?}"))?;

    let signature = BitString::from_bytes(&signer.sign(&blob).await?.to_bytes())
        .map_err(|e| format!("wrong Ed25519 signature length: {e:?}"))?;

    let certificate = builder
        .assemble(signature)
        .map_err(|e| format!("failed to assemble certificate: {e:?}"))?;

    let x509_certificate_string = certificate
        .to_pem(LineEnding::LF)
        .map_err(|e| format!("failed to encode certificate: {e:?}"))?;

    Ok(X509CertificateString {
        x509_certificate_string,
    })
}

fn validity() -> Validity {
    let not_before_system_time = std::time::SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_nanos(time()))
        .expect("failed to obtain current time");
    let ten_years = Duration::from_secs(10 * 365 * 24 * 60 * 60);
    Validity {
        not_before: Time::try_from(not_before_system_time).expect("failed to convert time"),
        not_after: Time::try_from(not_before_system_time + ten_years)
            .expect("failed to convert time"),
    }
}

#[update]
async fn root_ca_public_key() -> Result<PublicKeyReply, String> {
    let public_key_bytes = root_ca_public_key_bytes().await?;
    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&public_key_bytes),
    })
}

async fn root_ca_public_key_bytes() -> Result<Vec<u8>, String> {
    let request = ManagementCanisterSchnorrPublicKeyRequest {
        canister_id: None,
        derivation_path: vec![ic_cdk::api::caller().as_slice().to_vec()],
        key_id: key_id(),
    };

    let (res,): (ManagementCanisterSchnorrPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "schnorr_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("schnorr_public_key failed {}", e.1))?;

    Ok(res.public_key)
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PemCertificateRequest {
    pem_certificate_request: String,
}

#[update]
async fn child_certificate(
    certificate_request_info: PemCertificateRequest,
) -> Result<X509CertificateString, String> {
    let cert_req =
        CertReq::from_pem(certificate_request_info.pem_certificate_request.as_bytes())
            .map_err(|e| format!("failed to parse PEM certificate signing request: {e:?}"))?;

    verify_certificate_request_signature(&cert_req)?;

    prove_ownership(&cert_req, ic_cdk::api::caller() /*, ... */);

    let root_certificate_pem = root_ca_certificate().await?;
    let root_certificate =
        x509_cert::Certificate::from_pem(root_certificate_pem.x509_certificate_string.as_str())
            .map_err(|e| format!("failed to parse PEM root CA certificate: {e:?}"))?;

    let profile = Profile::Leaf {
        issuer: root_certificate.tbs_certificate.subject.clone(),
        enable_key_agreement: false,
        enable_key_encipherment: false,
    };

    // TODO: increment serial number
    let serial_number = SerialNumber::from(0u32);

    let signer = Signer::new().await?;

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity(),
        cert_req.info.subject.clone(),
        cert_req.info.public_key.clone(),
        &signer,
    )
    .expect("Create certificate");

    if !cert_req.info.attributes.is_empty() {
        return Err("Attributes are currently not supported in this example".to_string());
    }

    let blob = builder
        .finalize()
        .map_err(|e| format!("failed to finalize certificate builder: {e:?}"))?;

    let signature = BitString::from_bytes(&signer.sign(&blob).await?.to_bytes())
        .map_err(|e| format!("wrong Ed25519 signature length: {e:?}"))?;

    let certificate = builder
        .assemble(signature)
        .map_err(|e| format!("failed to assemble certificate: {e:?}"))?;

    let x509_certificate_string = certificate
        .to_pem(LineEnding::LF)
        .map_err(|e| format!("failed to encode certificate: {e:?}"))?;

    Ok(X509CertificateString {
        x509_certificate_string,
    })
}

fn verify_certificate_request_signature(certificate_request: &CertReq) -> Result<(), String> {
    match (
        certificate_request.algorithm.oid,
        &certificate_request.algorithm.parameters,
    ) {
        (ed25519::pkcs8::ALGORITHM_OID, None) => {
            use ed25519_dalek::{Verifier, VerifyingKey};
            let verifying_key = VerifyingKey::try_from(
                certificate_request
                    .info
                    .public_key
                    .subject_public_key
                    .raw_bytes(),
            )
            .map_err(|e| format!("malformed CSR public key: {e:?}"))?;
            let msg_buf = &mut vec![];
            certificate_request
                .info
                .encode_to_vec(msg_buf)
                .map_err(|e| format!("failed to encode CSR info: {e:?}"))?;
            let signature =
                ed25519_dalek::Signature::try_from(certificate_request.signature.raw_bytes())
                    .map_err(|e| format!("malformed CSR signature: {e:?}"))?;
            verifying_key
                .verify(msg_buf.as_slice(), &signature)
                .map_err(|e| format!("invalid CSR signature: {:?}", e))
        }
        // (ecdsa::ECDSA_SHA256_OID, Some(k256::Secp256k1::oid())) => {}
        _ => Err("unsupported algorithm".to_string()),
    }
}

fn prove_ownership(_cert_req: &CertReq, _caller: Principal /*, ... */) {
    // ********** This is a placeholder for a real implementation. **********
    // ********** In a real implementation, the subject would have **********
    // ********** to prove ownership of the subject name.          **********
}

getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

export_candid!();
