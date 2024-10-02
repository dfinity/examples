use candid::CandidType;
use serde::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

#[derive(CandidType, Serialize, Deserialize, Debug, Copy, Clone)]
pub enum SchnorrAlgorithm {
    #[serde(rename = "bip340secp256k1")]
    Bip340Secp256k1,
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct PublicKeyReply {
    pub public_key_hex: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SignatureReply {
    pub signature_hex: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SignatureVerificationReply {
    pub is_signature_valid: bool,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy, IntoStaticStr)]
pub enum KeyName {
    #[allow(unused)]
    #[strum(serialize = "dfx_test_key")]
    DfxTestKey,
    #[allow(unused)]
    #[strum(serialize = "test_key_1")]
    TestKey1,
    #[allow(unused)]
    #[strum(serialize = "key_1")]
    Key1,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CaKeyInformation {
    Ed25519(KeyName),
    EcdsaSecp256k1(KeyName),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct X509CertificateString {
    pub x509_certificate_string: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PemCertificateRequest {
    pub pem_certificate_request: String,
}
