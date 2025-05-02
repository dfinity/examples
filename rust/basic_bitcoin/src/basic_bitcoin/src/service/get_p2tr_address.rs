use crate::{
    p2tr::{self, p2tr_script_spend_info},
    BTC_CONTEXT, P2TR_DERIVATION_PATH_PREFIX,
};
use bitcoin::Address;
use ic_cdk::update;

/// Returns the P2TR address of this canister at the given derivation path. This
/// address uses two public keys:
/// 1) an internal key,
/// 2) a key that can be used to spend from a script.
///
/// The keys are derived by appending additional information to the provided
/// `derivation_path`.
#[update]
pub async fn get_p2tr_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair. To generate a user-specific address,
    // you would typically use a derivation path based on the user's identity or some other unique identifier.
    let derivation_path: Vec<Vec<u8>> = vec![P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];

    // Create the dual key pair for the P2TR address. The first key is the internal key,
    let (internal_key, script_path_key) = p2tr::get_public_keys(&ctx, derivation_path).await;

    // Converts a public key to a P2TR address. To compute the address, the public
    // key is tweaked with the taproot value, which is computed from the public key
    // and the Merkelized Abstract Syntax Tree (MAST, essentially a Merkle tree
    // containing scripts, in our case just one). Addresses are computed differently
    // for different Bitcoin networks.
    let taproot_spend_info = p2tr_script_spend_info(&internal_key, &script_path_key);

    Address::p2tr_tweaked(taproot_spend_info.output_key(), ctx.bitcoin_network).to_string()
}
