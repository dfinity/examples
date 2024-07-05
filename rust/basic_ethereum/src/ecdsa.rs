use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_crypto_ecdsa_secp256k1::{KeyDecodingError, PublicKey};
use ic_crypto_extended_bip32::{DerivationPath, ExtendedBip32DerivationResult};
use ic_ethereum_types::Address;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct EcdsaPublicKey {
    /// An ECDSA public key encoded in SEC1 compressed form.
    public_key: Vec<u8>,
    chain_code: Vec<u8>,
}

impl EcdsaPublicKey {
    pub fn derive_new_public_key(
        &self,
        derivation_path: &DerivationPath,
    ) -> ExtendedBip32DerivationResult<Self> {
        derivation_path
            .public_key_derivation(&self.public_key, &self.chain_code)
            .map(|output| Self {
                public_key: output.derived_public_key,
                chain_code: output.derived_chain_code,
            })
    }
}

impl From<EcdsaPublicKeyResponse> for EcdsaPublicKey {
    fn from(value: EcdsaPublicKeyResponse) -> Self {
        EcdsaPublicKey {
            public_key: value.public_key,
            chain_code: value.chain_code,
        }
    }
}

impl TryFrom<&EcdsaPublicKey> for PublicKey {
    type Error = KeyDecodingError;

    fn try_from(value: &EcdsaPublicKey) -> Result<Self, Self::Error> {
        PublicKey::deserialize_sec1(&value.public_key)
    }
}

impl TryFrom<&EcdsaPublicKey> for Address {
    type Error = KeyDecodingError;

    fn try_from(value: &EcdsaPublicKey) -> Result<Self, Self::Error> {
        PublicKey::try_from(value).map(|pk| ecdsa_public_key_to_address(&pk))
    }
}

fn ecdsa_public_key_to_address(pubkey: &PublicKey) -> Address {
    let key_bytes = pubkey.serialize_sec1(/*compressed=*/ false);
    debug_assert_eq!(key_bytes[0], 0x04);
    let hash = keccak(&key_bytes[1..]);
    let mut addr = [0u8; 20];
    addr[..].copy_from_slice(&hash[12..32]);
    Address::new(addr)
}

fn keccak(bytes: &[u8]) -> [u8; 32] {
    ic_crypto_sha3::Keccak256::hash(bytes)
}
