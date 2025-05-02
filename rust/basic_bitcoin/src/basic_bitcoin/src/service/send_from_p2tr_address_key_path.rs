use crate::{p2tr, SendRequest, BTC_CONTEXT, P2TR_DERIVATION_PATH_PREFIX};
use ic_cdk::update;

/// Sends the given amount of bitcoin from this canister's p2tr address to the given address.
/// Returns the transaction ID.
#[update]
pub async fn send_from_p2tr_address_key_path(request: SendRequest) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    let derivation_path = vec![P2TR_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];

    p2tr::send_key_path(
        &ctx,
        derivation_path,
        request.destination_address,
        request.amount_in_satoshi,
    )
    .await
    .to_string()
}
