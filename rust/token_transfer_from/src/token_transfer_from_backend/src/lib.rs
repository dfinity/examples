use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::msg_caller;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(msg_caller()),
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
    // `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
    let canister_id = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
        .expect("Could not decode the principal.");

    // Asynchronously call the ledger canister's `icrc1_transfer` method
    ic_cdk::call::Call::unbounded_wait(canister_id, "icrc2_transfer_from")
        // Provide the arguments for the call, here `transfer_args`
        .with_arg(transfer_from_args)
        // Await the completion of the asynchronous call, pausing the execution until the future is resolved.
        .await
        // Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
        // The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
        // otherwise, it unwraps the `Ok` value, allowing the chain to continue.
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        // Decode the response from the ledger canister, which is expected to be `Result<BlockIndex, TransferError>`.
        .candid::<Result<BlockIndex, TransferFromError>>()
        // Apply `map_err` again to handle any decoding errors, converting them into a string format for easier debugging.
        // The `?` operator is used again to propagate errors, ensuring that if the decoding fails, the function will return with that error.
        // Otherwise, it unwraps the `Ok` value, allowing the chain to continue.
        .map_err(|e| format!("failed to decode ledger response: {:?}", e))?
        // Use `map_err` again to handle any specific ledger transfer errors, converting them into a string format for easier debugging.
        .map_err(|e| format!("ledger transfer error {:?}", e))
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
