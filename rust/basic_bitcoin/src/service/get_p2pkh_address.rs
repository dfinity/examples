use crate::{common::DerivationPath, ecdsa::get_ecdsa_public_key, BTC_CONTEXT};
use bitcoin::{Address, PublicKey};
use ic_cdk::update;

/// Returns a legacy P2PKH (Pay-to-PubKey-Hash) address for this smart contract.
///
/// This address uses an ECDSA public key and encodes it in the legacy Base58 format.
/// It is supported by all bitcoin wallets and full nodes.
#[update]
pub async fn get_p2pkh_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    // Unique derivation paths are used for every address type generated, to ensure
    // each address has its own unique key pair.
    let derivation_path = DerivationPath::p2pkh(0, 0);

    // Get the ECDSA public key of this smart contract at the given derivation path
    let public_key = get_ecdsa_public_key(&ctx, derivation_path.to_vec_u8_path()).await;

    // Convert the public key to the format used by the Bitcoin library
    let public_key = PublicKey::from_slice(&public_key).unwrap();

    // Generate a legacy P2PKH address from the public key.
    // The address encoding (Base58) depends on the network type.
    Address::p2pkh(public_key, ctx.bitcoin_network).to_string()
}
