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

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Copy)]
#[derive(IntoStaticStr)]
pub enum SchnorrKeyName {
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
