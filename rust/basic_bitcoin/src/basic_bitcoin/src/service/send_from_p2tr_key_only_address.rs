use ic_cdk::update;

use crate::{common::DerivationPath, p2tr_key_only, SendRequest, BTC_CONTEXT};

/// Sends the given amount of bitcoin from this canister's p2tr address to the
/// given address. Returns the transaction ID.
///
/// IMPORTANT: This function uses an untweaked key as the spending key.
///
/// WARNING: This function is not suited for multi-party scenarios where
/// multiple keys are used for spending.
#[update]
pub async fn send_from_p2tr_key_only_address(request: SendRequest) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let derivation_path = DerivationPath::p2tr(0, 0);
    let tx_id = p2tr_key_only::send(
        &ctx,
        derivation_path.to_vec_u8_path(),
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await;

    tx_id.to_string()
}
