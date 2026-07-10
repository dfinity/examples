use candid::CandidType;
use serde::{Deserialize, Serialize};

/// The threshold signing key to use for the CA.
/// Pass the key name as a plain string, e.g.:
///   `(variant { Ed25519 = "test_key_1" })`
///
/// Available key names:
///   - `"test_key_1"` — mainnet test key (works on the local network too)
///   - `"key_1"`      — mainnet production key
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum CaKeyInformation {
    Ed25519(String),
    EcdsaSecp256k1(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct X509CertificateString {
    pub x509_certificate_string: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct PemCertificateRequest {
    pub pem_certificate_request: String,
}
