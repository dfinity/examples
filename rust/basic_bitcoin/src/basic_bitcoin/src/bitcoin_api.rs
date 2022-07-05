use ic_btc_types::{
    GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse,
    MillisatoshiPerByte, Network, Satoshi, SendTransactionRequest,
};
use ic_cdk::{api::call::call_with_payment, export::Principal};

// The fees for the various bitcoin endpoints.
const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
const GET_UTXOS_COST_CYCLES: u64 = 100_000_000;
const GET_CURRENT_FEE_PERCENTILES_CYCLES: u64 = 100_000_000;
const SEND_TRANSACTION_BASE_CYCLES: u64 = 5_000_000_000;
const SEND_TRANSACTION_PER_BYTE_CYCLES: u64 = 20_000_000;

/// Returns the balance of the given bitcoin address.
///
/// Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
pub async fn get_balance(network: Network, address: String) -> u64 {
    let balance_res: Result<(Satoshi,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_balance",
        (GetBalanceRequest {
            address,
            network,
            min_confirmations: None,
        },),
        GET_BALANCE_COST_CYCLES,
    )
    .await;

    balance_res.unwrap().0
}

/// Returns the UTXOs of the given bitcoin address.
///
/// NOTE: Pagination is ignored in this example. If an address has many thousands
/// of UTXOs, then subsequent calls to `bitcoin_get_utxos` are required.
///
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
pub async fn get_utxos(network: Network, address: String) -> GetUtxosResponse {
    let utxos_res: Result<(GetUtxosResponse,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_utxos",
        (GetUtxosRequest {
            address,
            network,
            filter: None,
        },),
        GET_UTXOS_COST_CYCLES,
    )
    .await;

    utxos_res.unwrap().0
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
///
/// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
pub async fn get_current_fee_percentiles(network: Network) -> Vec<MillisatoshiPerByte> {
    let res: Result<(Vec<MillisatoshiPerByte>,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_current_fee_percentiles",
        (GetCurrentFeePercentilesRequest { network },),
        GET_CURRENT_FEE_PERCENTILES_CYCLES,
    )
    .await;

    res.unwrap().0
}

/// Sends a (signed) transaction to the bitcoin network.
///
/// Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
pub async fn send_transaction(network: Network, transaction: Vec<u8>) {
    let transaction_fee = SEND_TRANSACTION_BASE_CYCLES
        + (transaction.len() as u64) * SEND_TRANSACTION_PER_BYTE_CYCLES;

    let res: Result<(), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            network,
            transaction,
        },),
        transaction_fee,
    )
    .await;

    res.unwrap();
}
