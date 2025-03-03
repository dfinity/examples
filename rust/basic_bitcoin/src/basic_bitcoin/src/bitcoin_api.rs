use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_utxos, BitcoinNetwork, GetUtxosRequest, GetUtxosResponse,
};

/// Returns the UTXOs of the given bitcoin address.
///
/// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(network: BitcoinNetwork, address: String) -> GetUtxosResponse {
    let filter = None;
    let utxos_res = bitcoin_get_utxos(GetUtxosRequest {
        address,
        network,
        filter,
    })
    .await;

    utxos_res.unwrap().0
}
