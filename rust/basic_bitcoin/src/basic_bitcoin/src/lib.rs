mod common;
mod types;
mod util;
use bitcoin::{util::psbt::serialize::Serialize as _, Address};
use ic_btc_types::{
    GetBalanceRequest, GetUtxosRequest, GetUtxosResponse, Network, SendTransactionRequest,
};
use ic_cdk::{api::call::call_with_payment, call, export::Principal, print, trap, export::candid::{CandidType, Deserialize}};
use ic_cdk_macros::update;
use std::str::FromStr;
use types::*;

const GET_BALANCE_COST_CYCLES: u64 = 100_000_000;
const GET_UTXOS_COST_CYCLES: u64 = 100_000_000;

// TODO: should this be an env variable?
const NETWORK: Network = Network::Regtest;

/// Returns the public key of this canister at derivation path [0].
#[update]
async fn get_public_key() -> Vec<u8> {
    let ecdsa_canister_id = Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap();

    #[allow(clippy::type_complexity)]
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
    let public_key = get_public_key().await;
    crate::util::p2pkh_address_from_public_key(public_key)
}

/// Returns the balance of the given bitcoin address.
#[update]
async fn get_balance(address: String) -> u64 {
    // A call to the `bitcoin_get_balance`, which retrieves the balance of a
    // bitcoin address.
    //
    // https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
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
#[update]
async fn get_utxos(address: String) -> GetUtxosResponse {
    // A call to the `bitcoin_get_utxos`, which retrieves the UTXOs of a
    // bitcoin address.
    //
    // https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
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

#[update]
async fn send_transaction(transaction: Vec<u8>) {
    let res: Result<(), _> = ic_cdk::api::call::call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            network: NETWORK,
            transaction,
        },),
        1_000_000_000_000,
    )
    .await;

    res.unwrap();
}

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    destination_address: String,
    amount_in_satoshi: u64
}

#[update]
pub async fn send(request: SendRequest) {
    let amount = request.amount_in_satoshi;
    let destination = request.destination_address;
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

    let spending_transaction = crate::common::build_transaction(
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
    let signed_transaction = crate::common::sign_transaction(
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
