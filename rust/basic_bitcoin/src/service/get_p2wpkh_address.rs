use crate::{common::DerivationPath, ecdsa::get_ecdsa_public_key, BTC_CONTEXT};
use bitcoin::{Address, CompressedPublicKey};
use ic_cdk::update;

/// Returns a native SegWit (P2WPKH) address for this smart contract.
///
/// This address uses a compressed ECDSA public key and is encoded in Bech32 (BIP-173).
/// It is widely supported and offers lower fees due to reduced witness data size.
#[update]
pub async fn get_p2wpkh_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair.
    let derivation_path = DerivationPath::p2wpkh(0, 0);

    // Get the ECDSA public key of this smart contract at the given derivation path
    let public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;

    // Create a CompressedPublicKey from the raw public key bytes
    let public_key = CompressedPublicKey::from_slice(&public_key).unwrap();

    // Generate a P2WPKH Bech32 address.
    // The network (mainnet, testnet, regtest) determines the HRP (e.g., "bc1" or "tb1").
    Address::p2wpkh(&public_key, ctx.bitcoin_network).to_string()
}
