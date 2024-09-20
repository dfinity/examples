use crate::SchnorrKeyName;

use super::SchnorrAlgorithm;
use candid::{CandidType, Principal};
use der::{asn1::BitString, pem::LineEnding, DecodePem, Encode, EncodePem};
use ic_cdk::export_candid;
use ic_cdk::{api::time, init, update};
use serde::{Deserialize, Serialize};
use spki::{AlgorithmIdentifier, AlgorithmIdentifierOwned, DynSignatureAlgorithmIdentifier};
use std::cell::OnceCell;
use std::convert::TryInto;
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

thread_local! {
    static KEY_NAME: RefCell<SchnorrKeyName> = RefCell::new(SchnorrKeyName::DfxTestKey);

    // cache the public key and certificate to avoid fetching them multiple times
    static ROOT_CA_PUBLIC_KEY: OnceCell<Vec<u8>> = OnceCell::new();
    static ROOT_CA_CERTIFICATE_PEM: OnceCell<String> = OnceCell::new();

    static CHILD_CERTIFICATE_SERIAL_NUMBER: RefCell<u32> = RefCell::new(0);
}

#[init]
fn init(key_name: SchnorrKeyName) {
    KEY_NAME.with(|state| {
        *state.borrow_mut() = key_name;
    });
}

#[update]
async fn root_ca_certificate() -> Result<X509CertificateString, String> {
    // if the certificate is already cached, return it

    if let Some(certificate) = ROOT_CA_CERTIFICATE_PEM.with(|inner| inner.get().map(|v| v.clone()))
    {
        return Ok(X509CertificateString {
            x509_certificate_string: certificate,
        });
    }

    // if the certificate is not cached, create it and try to cache it

    let serial_number = SerialNumber::from(0u32);

    let subject = Name::from_str(
        "CN=Web3 certification authority corporation,O=Web3 ceritifcation authority Inc,C=US",
    )
    .unwrap();

    let subject_public_key = der::asn1::BitString::new(0, root_ca_public_key_bytes().await?)
        .map_err(|e| format!("source: {:?}", e.source()))?;

    let pub_key = SubjectPublicKeyInfoOwned {
        algorithm: public_key_algorithm_identifier(),
        subject_public_key,
    };

    let newly_constructed_x509_certificate_string = pem_certificated_signed_by_root_ca(
        Profile::Root,
        serial_number,
        validity(),
        subject,
        pub_key,
    )
    .await
    .map_err(|e| format!("failed to create root certificate: {e:?}"))?;

    let x509_certificate_string = ROOT_CA_CERTIFICATE_PEM.with(move |inner| {
        inner
            .get_or_init(|| newly_constructed_x509_certificate_string)
            .clone()
    });

    Ok(X509CertificateString {
        x509_certificate_string,
    })
}

#[update]
async fn child_certificate(
    certificate_request_info: PemCertificateRequest,
) -> Result<X509CertificateString, String> {
    let cert_req =
        CertReq::from_pem(certificate_request_info.pem_certificate_request.as_bytes())
            .map_err(|e| format!("failed to parse PEM certificate signing request: {e:?}"))?;

    verify_certificate_request_signature(&cert_req)?;

    if !cert_req.info.attributes.is_empty() {
        return Err("Attributes are currently not supported in this example".to_string());
    }

    prove_ownership(&cert_req, ic_cdk::api::caller() /*, ... */)?;

    let root_certificate_pem = root_ca_certificate().await?;
    let root_certificate =
        x509_cert::Certificate::from_pem(root_certificate_pem.x509_certificate_string.as_str())
            .map_err(|e| format!("failed to parse PEM root CA certificate: {e:?}"))?;

    let profile = Profile::Leaf {
        issuer: root_certificate.tbs_certificate.subject.clone(),
        enable_key_agreement: false,
        enable_key_encipherment: false,
    };

    let serial_number = SerialNumber::from(next_child_certificate_serial_number());

    let x509_certificate_string = pem_certificated_signed_by_root_ca(
        profile,
        serial_number,
        // For simplicity of this example, let's just use the same validity
        // period as the root certificate. In a real application, the validity
        // would normally not start in the past and might end well before the root
        // certificate validity ends. Also, the validity of the child
        // ceritifcate should always be in the time frame of the root
        // certificate's validity.
        root_certificate.tbs_certificate.validity.clone(),
        cert_req.info.subject.clone(),
        cert_req.info.public_key.clone(),
    )
    .await
    .map_err(|e| format!("failed to create child certificate: {e:?}"))?;

    Ok(X509CertificateString {
        x509_certificate_string,
    })
}

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

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct X509CertificateString {
    x509_certificate_string: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PemCertificateRequest {
    pem_certificate_request: String,
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
            derivation_path: derivation_path(), // empty because there is only one root certificate for everyone in this example
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

async fn root_ca_public_key_bytes() -> Result<Vec<u8>, String> {
    // if the public key is already cached, return it
    if let Some(public_key) = ROOT_CA_PUBLIC_KEY.with(|inner| inner.get().map(|v| v.clone())) {
        return Ok(public_key);
    };

    // if the public key is not cached, fetch it from the management canister
    let request = ManagementCanisterSchnorrPublicKeyRequest {
        canister_id: None,
        derivation_path: derivation_path(),
        key_id: key_id(),
    };

    let (res,): (ManagementCanisterSchnorrPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "schnorr_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("schnorr_public_key failed {}", e.1))?;

    // try to initialize the cache with the fetched public key or returne the
    // cached value, because we were making an async call between the cache
    // check and cache initialization
    Ok(ROOT_CA_PUBLIC_KEY.with(move |inner| inner.get_or_init(|| res.public_key).clone()))
}

async fn pem_certificated_signed_by_root_ca(
    profile: Profile,
    serial_number: SerialNumber,
    validity: Validity,
    subject: Name,
    subject_public_key_info: SubjectPublicKeyInfoOwned,
) -> Result<String, String> {
    let signer = Signer::new().await?;

    let mut builder = CertificateBuilder::new(
        profile,
        serial_number,
        validity,
        subject,
        subject_public_key_info,
        &signer,
    )
    .expect("Create certificate");

    let blob = builder
        .finalize()
        .map_err(|e| format!("failed to finalize certificate builder: {e:?}"))?;

    let signature = BitString::from_bytes(&signer.sign(&blob).await?.to_bytes())
        .map_err(|e| format!("wrong Ed25519 signature length: {e:?}"))?;

    let certificate = builder
        .assemble(signature)
        .map_err(|e| format!("failed to assemble certificate: {e:?}"))?;

    assert_eq!(
        root_ca_public_key_bytes().await?.as_slice(),
        signer.public_key.0.as_slice()
    );

    certificate
        .to_pem(LineEnding::LF)
        .map_err(|e| format!("failed to encode certificate: {e:?}"))
}

fn verify_certificate_request_signature(certificate_request: &CertReq) -> Result<(), String> {
    let result = match (
        certificate_request.algorithm.oid,
        &certificate_request.algorithm.parameters,
    ) {
        (ed25519::pkcs8::ALGORITHM_OID, None) => {
            let msg_buf = &mut vec![];
            certificate_request
                .info
                .encode_to_vec(msg_buf)
                .map_err(|e| format!("failed to encode CSR info: {e:?}"))?;
            let public_key_bytes = certificate_request
                .info
                .public_key
                .subject_public_key
                .raw_bytes();
            verify_ed25519_signature(
                certificate_request.signature.raw_bytes(),
                msg_buf.as_slice(),
                public_key_bytes,
            )
        }
        // (ecdsa::ECDSA_SHA256_OID, Some(k256::Secp256k1::oid())) => {}
        _ => Err("unsupported algorithm".to_string()),
    };

    result.map_err(|e| format!("failed to verify CRS: {e:?}"))
}

fn verify_ed25519_signature(
    signature_bytes: &[u8],
    message_bytes: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};
    let verifying_key = VerifyingKey::from_bytes(
        public_key_bytes
            .try_into()
            .map_err(|e| format!("malformed public key: {e:?}"))?,
    )
    .map_err(|e| format!("couldn't create veryfing key: {e:?}"))?;
    let signature = Signature::from_slice(signature_bytes)
        .map_err(|e| format!("malformed signature: {e:?}"))?;
    verifying_key
        .verify(message_bytes, &signature)
        .map_err(|e| format!("invalid signature: {:?}", e))
}

fn next_child_certificate_serial_number() -> u32 {
    CHILD_CERTIFICATE_SERIAL_NUMBER.with(|state| {
        let mut serial_number = state.borrow_mut();
        *serial_number += 1;
        *serial_number
    })
}

fn prove_ownership(_cert_req: &CertReq, _caller: Principal /*, ... */) -> Result<(), String> {
    // ********** This is a placeholder for a real implementation. **********
    // ********** In a real implementation, the subject would have **********
    // ********** to prove ownership of the subject name.          **********
    Ok(())
}

fn key_id() -> SchnorrKeyId {
    KEY_NAME.with(|state| {
        let name = String::from(<&'static str>::from(*state.borrow()));
        SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name,
        }
    })
}

fn derivation_path() -> Vec<Vec<u8>> {
    vec![]
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

fn public_key_algorithm_identifier() -> AlgorithmIdentifier<der::Any> {
    AlgorithmIdentifier::<der::Any> {
        oid: ed25519::pkcs8::ALGORITHM_OID,
        parameters: None,
    }
}

getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

export_candid!();
