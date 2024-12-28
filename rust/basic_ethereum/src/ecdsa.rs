use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_crypto_ecdsa_secp256k1::PublicKey;
use ic_crypto_extended_bip32::{DerivationPath, ExtendedBip32DerivationResult};
use ic_ethereum_types::Address;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EcdsaPublicKey {
    public_key: PublicKey,
    chain_code: Vec<u8>,
}

impl EcdsaPublicKey {
    pub fn derive_new_public_key(
        &self,
        derivation_path: &DerivationPath,
    ) -> ExtendedBip32DerivationResult<Self> {
        derivation_path
            .public_key_derivation(
                &self.public_key.serialize_sec1(/*compressed=*/ true),
                &self.chain_code,
            )
            .map(|output| Self {
                public_key: PublicKey::deserialize_sec1(&output.derived_public_key)
                    .expect("BUG: invalid public key"),
                chain_code: output.derived_chain_code,
            })
    }
}

impl AsRef<PublicKey> for EcdsaPublicKey {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
    }
}

impl From<EcdsaPublicKeyResponse> for EcdsaPublicKey {
    fn from(value: EcdsaPublicKeyResponse) -> Self {
        EcdsaPublicKey {
            public_key: PublicKey::deserialize_sec1(&value.public_key)
                .expect("BUG: invalid public key"),
            chain_code: value.chain_code,
        }
    }
}

impl From<&EcdsaPublicKey> for Address {
    fn from(value: &EcdsaPublicKey) -> Self {
        let key_bytes = value.as_ref().serialize_sec1(/*compressed=*/ false);
        debug_assert_eq!(key_bytes[0], 0x04);
        let hash = ic_crypto_sha3::Keccak256::hash(&key_bytes[1..]);
        let mut addr = [0u8; 20];
        addr[..].copy_from_slice(&hash[12..32]);
        Address::new(addr)
    }
}
