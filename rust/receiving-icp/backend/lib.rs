use candid::Principal;
use ic_ledger_types::{AccountBalanceArgs, AccountIdentifier, Subaccount};

// The ledger principal is baked in at deploy time from the ICP_LEDGER_CANISTER_ID
// environment variable, which icp.yaml configures per environment:
//   local / production: ryjl3-tyaaa-aaaaa-aaaba-cai  (ICP ledger)
//   staging:            xafvr-biaaa-aaaai-aql5q-cai  (TESTICP ledger)
//
// See icp.yaml for the full environment configuration.
const LEDGER_PRINCIPAL: &str = env!("ICP_LEDGER_CANISTER_ID");

fn get_account(upper: u128, lower: u128) -> AccountIdentifier {
    // Create a 32-byte array by combining the little endian representation of upper and lower.
    let mut subaccount_bytes = [0u8; 32];
    subaccount_bytes[0..16].copy_from_slice(&upper.to_le_bytes());
    subaccount_bytes[16..32].copy_from_slice(&lower.to_le_bytes());
    AccountIdentifier::new(&ic_cdk::api::canister_self(), &Subaccount(subaccount_bytes))
}

/// Retrieves the canister's main account identifier.
#[ic_cdk::query]
async fn account() -> String {
    get_account(0, 0).to_string()
}

/// Retrieves an account identifier for a specific subaccount.
#[ic_cdk::query]
async fn subaccount(upper: u128, lower: u128) -> String {
    get_account(upper, lower).to_string()
}

/// Retrieves the canister's ICP balance from the ledger.
#[ic_cdk::update]
async fn get_balance() -> u64 {
    get_balance_of_subaccount(0, 0).await
}

/// Retrieves the ICP balance of a specific subaccount from the ledger.
#[ic_cdk::update]
async fn get_balance_of_subaccount(upper: u128, lower: u128) -> u64 {
    let ledger = Principal::from_text(LEDGER_PRINCIPAL).expect("invalid ledger principal");
    let account = get_account(upper, lower);
    let balance = ic_ledger_types::account_balance(ledger, &AccountBalanceArgs { account })
        .await
        .expect("call to get balance failed");

    balance.e8s()
}
