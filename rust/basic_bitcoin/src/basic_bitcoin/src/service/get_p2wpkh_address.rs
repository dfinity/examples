use crate::{ecdsa::get_ecdsa_public_key, BTC_CONTEXT, P2WPKH_DERIVATION_PATH_PREFIX};
use bitcoin::{Address, CompressedPublicKey};
use ic_cdk::update;

/// Returns the P2WPKH (Segwit) address of this canister
#[update]
pub async fn get_p2wpkh_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair. To generate a user-specific address,
    // you would typically use a derivation path based on the user's identity or some other unique identifier.
    let derivation_path: Vec<Vec<u8>> = vec![P2WPKH_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];

    // Get the ECDSA public key of this canister at the given derivation path
    let public_key = get_ecdsa_public_key(&ctx, derivation_path).await;

    // Create a CompressedPublicKey from the raw public key bytes
    let public_key = CompressedPublicKey::from_slice(&public_key).unwrap();

    // Generate a P2WPKH address from the public key
    Address::p2wpkh(&public_key, ctx.bitcoin_network).to_string()
}
