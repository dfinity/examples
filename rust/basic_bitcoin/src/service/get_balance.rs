use crate::BTC_CONTEXT;
use ic_cdk::{
    bitcoin_canister::{bitcoin_get_balance, GetBalanceRequest},
    update,
};

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    bitcoin_get_balance(&GetBalanceRequest {
        address,
        network: ctx.network,
        min_confirmations: None,
    })
    .await
    .unwrap()
}
