use crate::BTC_CONTEXT;
use ic_cdk::{
    bitcoin_canister::{bitcoin_get_utxos, GetUtxosRequest, GetUtxosResponse},
    update,
};

/// Returns the UTXOs of the given Bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    bitcoin_get_utxos(&GetUtxosRequest {
        address,
        network: ctx.network,
        filter: None,
    })
    .await
    .unwrap()
}
