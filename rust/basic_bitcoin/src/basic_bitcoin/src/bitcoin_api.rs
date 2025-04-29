use ic_cdk::bitcoin_canister::{
    bitcoin_get_balance, bitcoin_get_current_fee_percentiles, bitcoin_get_utxos,
    bitcoin_send_transaction, GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest,
    GetUtxosResponse, MillisatoshiPerByte, Network, SendTransactionRequest,
};

/// Returns the balance of the given bitcoin address.
///
/// Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn get_balance(network: Network, address: String) -> u64 {
    let min_confirmations = None;
    let balance_res = bitcoin_get_balance(&GetBalanceRequest {
        address,
        network,
        min_confirmations,
    })
    .await;

    balance_res.unwrap()
}

/// Returns the UTXOs of the given bitcoin address.
///
/// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(network: Network, address: String) -> GetUtxosResponse {
    let filter = None;
    let utxos_res = bitcoin_get_utxos(&GetUtxosRequest {
        address,
        network,
        filter,
    })
    .await;

    utxos_res.unwrap()
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
pub async fn get_current_fee_percentiles(network: Network) -> Vec<MillisatoshiPerByte> {
    let res =
        bitcoin_get_current_fee_percentiles(&GetCurrentFeePercentilesRequest { network }).await;

    res.unwrap()
}

/// Sends a (signed) transaction to the bitcoin network.
///
/// Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(network: Network, transaction: Vec<u8>) {
    let res = bitcoin_send_transaction(&SendTransactionRequest {
        network,
        transaction,
    })
    .await;

    res.unwrap();
}
