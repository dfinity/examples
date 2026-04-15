use crate::BTC_CONTEXT;
use ic_cdk::update;
use ic_cdk_bitcoin_canister::{bitcoin_get_balance, GetBalanceRequest};

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    bitcoin_get_balance(&GetBalanceRequest {
        address,
        network: ctx.network.into(),
        min_confirmations: None,
    })
    .await
    .unwrap()
}
