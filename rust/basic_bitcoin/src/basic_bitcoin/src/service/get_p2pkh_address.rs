use crate::{ecdsa::get_ecdsa_public_key, BTC_CONTEXT, P2PKH_DERIVATION_PATH_PREFIX};
use bitcoin::{Address, PublicKey};
use ic_cdk::update;

/// Returns the P2PKH address of this canister
#[update]
pub async fn get_p2pkh_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair. To generate a user-specific address,
    // you would typically use a derivation path based on the user's identity or some other unique identifier.
    let derivation_path: Vec<Vec<u8>> = vec![P2PKH_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];

    // Get the ECDSA public key of this canister at the given derivation path
    let public_key = get_ecdsa_public_key(&ctx, derivation_path).await;

    // Convert the public key to the format used by the Bitcoin library
    let public_key = PublicKey::from_slice(&public_key).unwrap();

    // Generate a P2PKH address from the public key
    Address::p2pkh(public_key, ctx.bitcoin_network).to_string()
}
