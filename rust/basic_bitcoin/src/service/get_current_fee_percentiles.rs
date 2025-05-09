use crate::BTC_CONTEXT;
use ic_cdk::{
    bitcoin_canister::{
        bitcoin_get_current_fee_percentiles, GetCurrentFeePercentilesRequest, MillisatoshiPerByte,
    },
    update,
};

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest {
        network: ctx.network,
    })
    .await
    .unwrap()
}
