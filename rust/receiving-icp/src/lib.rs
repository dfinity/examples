use candid::Principal;
use ic_ledger_types::{AccountBalanceArgs, AccountIdentifier, Subaccount};

// This is the ledger principal for TESTICP
// To use real ICP, use `ryjl3-tyaaa-aaaaa-aaaba-cai` instead.
const LEDGER_PRINCIPAL: &str = "xafvr-biaaa-aaaai-aql5q-cai";

/// Retrieves the canister's account identifier.
#[ic_cdk::query]
async fn account_identifier() -> String {
    AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount([0; 32])).to_string()
}

/// Retrieves the canister's principal.
#[ic_cdk::query]
async fn principal() -> Principal {
    ic_cdk::api::canister_self()
}

/// Retrieves own balance from the ledger.
#[ic_cdk::update]
async fn get_balance() -> u64 {
    let ledger = Principal::from_text(LEDGER_PRINCIPAL).expect("invalid ledger principal");

    // Compute the canister's account identifier.
    let account = AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount([0; 32]));

    // Retrieves the account's balance from the ledger.
    let balance = ic_ledger_types::account_balance(ledger, &AccountBalanceArgs { account })
        .await
        .expect("call to get balance failed");

    balance.e8s()
}
