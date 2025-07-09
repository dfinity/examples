use crate::{CaKeyInformation, KeyName, PemCertificateRequest, X509CertificateString};

use super::SchnorrAlgorithm;
use candid::{CandidType, Principal};
use der::{asn1::BitString, pem::LineEnding, DecodePem, Encode, EncodePem};
use ic_cdk::api::management_canister::ecdsa as cdk_ecdsa;
use ic_cdk::export_candid;
use ic_cdk::{api::time, init, update};
use pkcs8::AssociatedOid;
use serde::{Deserialize, Serialize};
use signature::Keypair;
use spki::{AlgorithmIdentifier, DynSignatureAlgorithmIdentifier, EncodePublicKey};
use std::{
    cell::OnceCell, cell::RefCell, convert::TryFrom, convert::TryInto, error::Error, ops::Deref,
    str::FromStr, time::Duration,
};
use x509_cert::{
    builder::{Builder, CertificateBuilder, Profile},
    name::Name,
    request::CertReq,
    serial_number::SerialNumber,
    spki::SubjectPublicKeyInfoOwned,
    time::{Time, Validity},
};

mod signer;
use signer::{EcdsaSecp256k1Signer, Ed25519Signer, Sign};

type CanisterId = Principal;

impl TryFrom<&CaKeyInformation> for SchnorrKeyId {
    type Error = String;

    fn try_from(value: &CaKeyInformation) -> Result<Self, Self::Error> {
        match value {
            CaKeyInformation::Ed25519(key_name) => Ok(SchnorrKeyId {
                algorithm: SchnorrAlgorithm::Ed25519,
                name: String::from(<&'static str>::from(key_name)),
            }),
            something_else => Err(format!(
                "Expected Ed25519 CA key but got {something_else:?}"
            )),
        }
    }
}

impl TryFrom<&CaKeyInformation> for cdk_ecdsa::EcdsaKeyId {
    type Error = String;

    fn try_from(value: &CaKeyInformation) -> Result<Self, Self::Error> {
        match value {
            CaKeyInformation::EcdsaSecp256k1(key_name) => Ok(cdk_ecdsa::EcdsaKeyId {
                curve: cdk_ecdsa::EcdsaCurve::Secp256k1,
                name: String::from(<&'static str>::from(key_name)),
            }),
            something_else => Err(format!(
                "Expected EcdsaSecp256k1 CA key but got {something_else:?}"
            )),
        }
    }
}

thread_local! {
    static CA_KEY_INFORMATION: RefCell<CaKeyInformation> = RefCell::new(CaKeyInformation::Ed25519(KeyName::DfxTestKey));

    // cache the public key and certificate to avoid fetching them multiple times
    static ROOT_CA_PUBLIC_KEY: OnceCell<Vec<u8>> = OnceCell::new();
    static ROOT_CA_CERTIFICATE_PEM: OnceCell<String> = OnceCell::new();

    static CHILD_CERTIFICATE_SERIAL_NUMBER: RefCell<u32> = RefCell::new(1);
}

#[init]
fn init(ca_key_information: CaKeyInformation) {
    CA_KEY_INFORMATION.with(|value| {
        *value.borrow_mut() = ca_key_information;
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

    let serial_number = SerialNumber::from(1u32);

    let subject = Name::from_str(
        "CN=Web3 certification authority corporation,O=Web3 ceritifcation authority Inc,C=US",
    )
    .unwrap();

    let newly_constructed_x509_certificate_string = match CA_KEY_INFORMATION
        .with(|value| *value.borrow())
    {
        CaKeyInformation::Ed25519(_) => {
            let signer = Ed25519Signer::new()
                .await
                .map_err(|e| format!("failed to create Ed25519 signer: {e:?}"))?;

            let subject_public_key =
                der::asn1::BitString::new(0, root_ca_public_key_bytes().await?)
                    .map_err(|e| format!("source: {:?}", e.source()))?;

            let pub_key = SubjectPublicKeyInfoOwned {
                algorithm: signer.signature_algorithm_identifier().unwrap(),
                subject_public_key,
            };

            pem_certificate_signed_by_root_ca(
                Profile::Root,
                serial_number,
                validity(),
                subject,
                pub_key,
                signer,
            )
            .await
            .map_err(|e| format!("failed to create root certificate: {e:?}"))?
        }
        CaKeyInformation::EcdsaSecp256k1(_) => {
            let signer = EcdsaSecp256k1Signer::new()
                .await
                .map_err(|e| format!("failed to create ECDSA secp256k1 signer: {e:?}"))?;

            let public_key_bytes_compressed = root_ca_public_key_bytes().await?;
            let public_key_bytes_uncompressed =
                k256::ecdsa::VerifyingKey::from_sec1_bytes(public_key_bytes_compressed.as_slice())
                    .map_err(|e| format!("malformed public key: {e:?}"))?
                    .to_encoded_point(false);

            let subject_public_key =
                der::asn1::BitString::new(0, public_key_bytes_uncompressed.as_bytes())
                    .map_err(|e| format!("source: {:?}", e.source()))?;

            let pub_key = SubjectPublicKeyInfoOwned {
                algorithm: AlgorithmIdentifier {
                    // Public Key Algorithm: id-ecPublicKey (1.2.840.10045.2.1)
                    oid: pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1"),
                    parameters: Some(der::Any::from(k256::Secp256k1::OID)),
                },
                subject_public_key,
            };

            pem_certificate_signed_by_root_ca(
                Profile::Root,
                serial_number,
                validity(),
                subject,
                pub_key,
                signer,
            )
            .await
            .map_err(|e| format!("failed to create root certificate: {e:?}"))?
        }
    };

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

    // For simplicity of this example, let's just use the same validity
    // period as the root certificate. In a real application, the validity
    // would normally not start in the past and might end well before the root
    // certificate validity ends. Also, the validity of the child
    // ceritifcate should always be in the time frame of the root
    // certificate's validity.
    let validity = root_certificate.tbs_certificate.validity.clone();

    let x509_certificate_string = {
        match CA_KEY_INFORMATION.with(|value| *value.borrow()) {
            CaKeyInformation::Ed25519(_) => {
                let signer = Ed25519Signer::new()
                    .await
                    .map_err(|e| format!("failed to create Ed25519 signer: {e:?}"))?;
                pem_certificate_signed_by_root_ca(
                    profile,
                    serial_number,
                    validity,
                    cert_req.info.subject.clone(),
                    cert_req.info.public_key.clone(),
                    signer,
                )
                .await
                .map_err(|e| format!("failed to create child certificate: {e:?}"))?
            }
            CaKeyInformation::EcdsaSecp256k1(_) => {
                let signer = EcdsaSecp256k1Signer::new()
                    .await
                    .map_err(|e| format!("failed to create Ed25519 signer: {e:?}"))?;
                pem_certificate_signed_by_root_ca(
                    profile,
                    serial_number,
                    validity,
                    cert_req.info.subject.clone(),
                    cert_req.info.public_key.clone(),
                    signer,
                )
                .await
                .map_err(|e| format!("failed to create child certificate: {e:?}"))?
            }
        }
    };

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

async fn root_ca_public_key_bytes() -> Result<Vec<u8>, String> {
    // if the public key is already cached, return it
    if let Some(public_key) = ROOT_CA_PUBLIC_KEY.with(|inner| inner.get().map(|v| v.clone())) {
        return Ok(public_key);
    };

    let result = match CA_KEY_INFORMATION.with(|value| *value.borrow()) {
        CaKeyInformation::Ed25519(_) => {
            // if the public key is not cached, fetch it from the management canister
            let request = ManagementCanisterSchnorrPublicKeyRequest {
                canister_id: None,
                derivation_path: derivation_path(),
                key_id: CA_KEY_INFORMATION
                    .with(|value| SchnorrKeyId::try_from(value.borrow().deref()))?,
            };

            let (res,): (ManagementCanisterSchnorrPublicKeyReply,) = ic_cdk::call(
                Principal::management_canister(),
                "schnorr_public_key",
                (request,),
            )
            .await
            .map_err(|e| format!("schnorr_public_key failed {}", e.1))?;

            res.public_key
        }
        CaKeyInformation::EcdsaSecp256k1(_) => {
            let args = cdk_ecdsa::EcdsaPublicKeyArgument {
                canister_id: None,
                derivation_path: derivation_path(),
                key_id: CA_KEY_INFORMATION
                    .with(|value| cdk_ecdsa::EcdsaKeyId::try_from(value.borrow().deref()))?,
            };
            let response = cdk_ecdsa::ecdsa_public_key(args)
                .await
                .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;
            response.0.public_key
        }
    };

    // try to initialize the cache with the fetched public key or returne the
    // cached value, because we were making an async call between the cache
    // check and cache initialization
    Ok(ROOT_CA_PUBLIC_KEY.with(move |inner| inner.get_or_init(|| result).clone()))
}

async fn pem_certificate_signed_by_root_ca<Signer>(
    profile: Profile,
    serial_number: SerialNumber,
    validity: Validity,
    subject: Name,
    subject_public_key_info: SubjectPublicKeyInfoOwned,
    signer: Signer,
) -> Result<String, String>
where
    Signer: Sign,
    Signer: Keypair + DynSignatureAlgorithmIdentifier,
    Signer::VerifyingKey: EncodePublicKey,
{
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

    let signature = BitString::from_bytes(&signer.sign(&blob).await?)
        .map_err(|e| format!("wrong signature length: {e:?}"))?;

    let certificate = builder
        .assemble(signature)
        .map_err(|e| format!("failed to assemble certificate: {e:?}"))?;

    certificate
        .to_pem(LineEnding::LF)
        .map_err(|e| format!("failed to encode certificate: {e:?}"))
}

fn verify_certificate_request_signature(certificate_request: &CertReq) -> Result<(), String> {
    fn certificate_as_message_and_public_key_bytes(
        certificate_request: &CertReq,
    ) -> (Vec<u8>, Vec<u8>) {
        let mut message = vec![];
        certificate_request
            .info
            .encode(&mut message)
            .expect("failed to encode certificate request info");

        let public_key_bytes = certificate_request
            .info
            .public_key
            .subject_public_key
            .raw_bytes()
            .to_vec();

        (message, public_key_bytes)
    }
    let result = match &certificate_request.algorithm {
        AlgorithmIdentifier::<der::Any> {
            oid: ed25519::pkcs8::ALGORITHM_OID,
            parameters: None,
        } => {
            let (message, public_key_bytes) =
                certificate_as_message_and_public_key_bytes(certificate_request);
            verify_ed25519_signature(
                certificate_request.signature.raw_bytes(),
                message.as_slice(),
                public_key_bytes.as_slice(),
            )
        }
        AlgorithmIdentifier::<der::Any> {
            oid: ecdsa::ECDSA_SHA256_OID,
            parameters: None, // secp256k1 is non-standard and is sometimes encoded as None
        } => {
            let (message, public_key_bytes) =
                certificate_as_message_and_public_key_bytes(certificate_request);
            verify_ecdsa_secp256k1_signature(
                certificate_request.signature.raw_bytes(),
                message.as_slice(),
                public_key_bytes.as_slice(),
            )
        }
        _ => Err(format!(
            "unsupported algorithm: {:?}",
            certificate_request.algorithm
        )),
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

fn verify_ecdsa_secp256k1_signature(
    der_signature_bytes: &[u8],
    message_bytes: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), String> {
    use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
    let verifying_key = VerifyingKey::from_sec1_bytes(
        public_key_bytes
            .try_into()
            .map_err(|e| format!("malformed public key: {e:?}"))?,
    )
    .map_err(|e| format!("couldn't create veryfing key: {e:?}"))?;

    let mut signature = Signature::from_der(der_signature_bytes)
        .map_err(|e| format!("malformed signature: {e:?}"))?;
    if let Some(normalized_signature) = signature.normalize_s() {
        signature = normalized_signature;
    };
    let result = verifying_key
        .verify(message_bytes, &signature)
        .map_err(|e| format!("invalid signature: {:?}", e));
    result
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

getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

export_candid!();
