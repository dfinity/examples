use crate::{p2tr, SendRequest, BTC_CONTEXT};
use ic_cdk::update;

#[update]
pub async fn send_from_p2tr_address_script_path(request: SendRequest) -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    p2tr::send_script_path(&ctx, request.destination_address, request.amount_in_satoshi)
        .await
        .to_string()
}
