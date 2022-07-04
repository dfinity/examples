mod types;
mod util;
use bitcoin::{util::psbt::serialize::Serialize as _, Address};
use ic_btc_types::{
    GetBalanceRequest, GetCurrentFeePercentilesRequest, GetUtxosRequest, GetUtxosResponse,
    MillisatoshiPerByte, Network, SendTransactionRequest,
};
use ic_cdk::{api::call::call_with_payment, call, export::Principal, print, trap};
use ic_cdk_macros::update;
use std::str::FromStr;
use types::*;

const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
const GET_UTXOS_COST_CYCLES: u64 = 100_000_000;
const GET_CURRENT_FEE_PERCENTILES_FEE: u64 = 100_000_000;

// TODO: should this be an env variable?
const NETWORK: Network = Network::Regtest;

/// Returns the balance of the given bitcoin address.
///
/// Relies on the `bitcoin_get_balance` endpoint.
/// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
#[update]
async fn get_balance(address: String) -> u64 {
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
async fn get_utxos(address: String) -> GetUtxosResponse {
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
async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
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
async fn send_transaction(transaction: Vec<u8>) {
    let res: Result<(), _> = ic_cdk::api::call::call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            network: NETWORK,
            transaction,
        },),
        1_000_000_000_000, // TODO: fix the fees
    )
    .await;

    res.unwrap();
}

/// Returns the public key of this canister at derivation path [0].
#[update]
async fn get_public_key() -> Vec<u8> {
    let ecdsa_canister_id = Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();

    // Retrieve the public key of this canister at derivation path [0]
    // from the ECDSA API.
    let res: (ECDSAPublicKeyReply,) = call(
        ecdsa_canister_id,
        "ecdsa_public_key",
        (ECDSAPublicKey {
            canister_id: None,
            derivation_path: vec![vec![0]],
            key_id: EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: String::from("test"),
            },
        },),
    )
    .await
    .unwrap();

    res.0.public_key
}

/// Returns the P2PKH address of this canister at derivation path [0].
#[update]
async fn get_p2pkh_address() -> String {
    // Fetch our public key.
    let public_key = get_public_key().await;

    // Compute the P2PKH address from our public key.
    crate::util::p2pkh_address_from_public_key(public_key)
}

#[update]
pub async fn send(request: SendRequest) {
    let amount = request.amount_in_satoshi;
    let destination = request.destination_address;

    // TODO: compute the fees from the fees api.
    let fees: u64 = 10_000;

    if amount <= fees {
        trap("Amount must be higher than the fee of 10,000 satoshis")
    }

    let destination = match Address::from_str(&destination) {
        Ok(destination) => destination,
        Err(_) => trap("Invalid destination address"),
    };

    let our_address = get_p2pkh_address().await;

    print(&format!("BTC address: {}", our_address));

    // Fetch our UTXOs.
    let utxos = get_utxos(our_address.clone()).await.utxos;

    let spending_transaction = crate::util::build_transaction(
        utxos,
        Address::from_str(&our_address).unwrap(),
        destination,
        amount,
        fees,
    )
    .unwrap_or_else(|err| {
        trap(&format!("Error building transaction: {}", err));
    });

    let tx_bytes = spending_transaction.serialize();
    print(&format!("Transaction to sign: {}", hex::encode(tx_bytes)));

    // Sign transaction
    let signed_transaction = crate::util::sign_transaction(
        spending_transaction,
        Address::from_str(&our_address).unwrap(),
        get_public_key().await,
    )
    .await;

    let signed_transaction_bytes = signed_transaction.serialize();
    print(&format!(
        "Signed transaction: {}",
        hex::encode(signed_transaction_bytes.clone())
    ));

    print("Sending transaction");

    send_transaction(signed_transaction_bytes).await;
    print("Done");
}
