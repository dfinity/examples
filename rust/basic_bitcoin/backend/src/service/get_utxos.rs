use crate::BTC_CONTEXT;
use ic_cdk::update;
use ic_cdk_bitcoin_canister::{bitcoin_get_utxos, GetUtxosRequest, GetUtxosResponse};

/// Returns the UTXOs of the given Bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    bitcoin_get_utxos(&GetUtxosRequest {
        address,
        network: ctx.network.into(),
        filter: None,
    })
    .await
    .unwrap()
}
