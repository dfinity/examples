use candid::{CandidType, Principal};

use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    amount: Tokens,
    to_principal: Principal,
    to_subaccount: Option<Subaccount>,
}

#[update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to principal {} subaccount {:?}",
        &args.amount,
        &args.to_principal,
        &args.to_subaccount
    );
    let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: args.amount,
        fee: Tokens::from_e8s(10_000),
        // The subaccount of the account identifier that will be used to withdraw tokens and send them
        // to another account identifier. If set to None then the default subaccount will be used.
        // See the [Ledger doc](https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/#accounts).
        from_subaccount: None,
        to: AccountIdentifier::new(&args.to_principal, &to_subaccount),
        created_at_time: None,
    };
    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e))
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
