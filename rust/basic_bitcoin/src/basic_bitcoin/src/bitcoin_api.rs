use ic_btc_types::{
    GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse,
    MillisatoshiPerByte, Network, SendTransactionRequest,
};
use ic_cdk::{api::call::call_with_payment, export::Principal};
use ic_cdk_macros::update;

// The bitcoin network to connect to.
//
// When developing locally this should be `Regtest`.
// When deploying to the IC this should be `Testnet`.
const NETWORK: Network = Network::Regtest;

// The fees for the various bitcoin endpoints.
const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
const GET_UTXOS_COST_CYCLES: u64 = 100_000_000;
const GET_CURRENT_FEE_PERCENTILES_FEE: u64 = 100_000_000;
const SEND_TRANSACTION_BASE_FEE: u64 = 5_000_000_000;
const SEND_TRANSACTION_FEE_PER_BYTE: u64 = 20_000_000;

/// Returns the balance of the given bitcoin address.
///
/// Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
#[update]
pub async fn get_balance(address: String) -> u64 {
    let balance_res: Result<(u64,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_balance",
        (GetBalanceRequest {
            address,
            network: NETWORK,
            min_confirmations: None,
        },),
        GET_BALANCE_COST_CYCLES,
    )
    .await;

    balance_res.unwrap().0
}

/// Returns the UTXOs of the given bitcoin address.
///
/// Relies on the `bitcoin_get_utxos` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let utxos_res: Result<(GetUtxosResponse,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_utxos",
        (GetUtxosRequest {
            address,
            network: NETWORK,
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
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let res: Result<(Vec<MillisatoshiPerByte>,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_current_fee_percentiles",
        (GetCurrentFeePercentilesRequest { network: NETWORK },),
        GET_CURRENT_FEE_PERCENTILES_FEE,
    )
    .await;

    res.unwrap().0
}

/// Sends a (signed) transaction to the bitcoin network.
///
/// Relies on the `bitcoin_send_transaction` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
#[update]
pub async fn send_transaction(transaction: Vec<u8>) {
    let transaction_fee =
        SEND_TRANSACTION_BASE_FEE + (transaction.len() as u64) * SEND_TRANSACTION_FEE_PER_BYTE;

    let res: Result<(), _> = ic_cdk::api::call::call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            network: NETWORK,
            transaction,
        },),
        transaction_fee,
    )
    .await;

    res.unwrap();
}
