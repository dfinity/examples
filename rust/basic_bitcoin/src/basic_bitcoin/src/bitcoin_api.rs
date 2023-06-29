use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_balance, bitcoin_get_current_fee_percentiles, bitcoin_get_utxos,
    bitcoin_send_transaction, BitcoinNetwork, GetBalanceRequest, GetCurrentFeePercentilesRequest,
    GetUtxosRequest, GetUtxosResponse, MillisatoshiPerByte, Satoshi, SendTransactionRequest,
};

/// Returns the balance of the given bitcoin address.
///
/// Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn get_balance(network: BitcoinNetwork, address: String) -> u64 {
    let balance_res: CallResult<(Satoshi,)> = bitcoin_get_balance(GetBalanceRequest {
        address,
        network,
        min_confirmations: None,
    })
    .await;

    balance_res.unwrap().0
}

/// Returns the UTXOs of the given bitcoin address.
///
/// NOTE: Pagination is ignored in this example. If an address has many thousands
/// of UTXOs, then subsequent calls to `bitcoin_get_utxos` are required.
///
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(network: BitcoinNetwork, address: String) -> GetUtxosResponse {
    let utxos_res: Result<(GetUtxosResponse,), _> = bitcoin_get_utxos(GetUtxosRequest {
        address,
        network,
        filter: None,
    })
    .await;

    utxos_res.unwrap().0
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
pub async fn get_current_fee_percentiles(network: BitcoinNetwork) -> Vec<MillisatoshiPerByte> {
    let res: Result<(Vec<MillisatoshiPerByte>,), _> =
        bitcoin_get_current_fee_percentiles(GetCurrentFeePercentilesRequest { network }).await;

    res.unwrap().0
}

/// Sends a (signed) transaction to the bitcoin network.
///
/// Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(network: BitcoinNetwork, transaction: Vec<u8>) {
    let res: Result<(), _> = bitcoin_send_transaction(SendTransactionRequest {
        network,
        transaction,
    })
    .await;

    res.unwrap();
}
