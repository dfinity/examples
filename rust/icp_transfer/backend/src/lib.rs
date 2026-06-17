use candid::Principal;
use ic_cdk::{query, update};
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

// Convert a principal and optional subaccount to its AccountIdentifier as a
// lowercase hex string — the format shown in block explorers and CEX deposit screens.
#[query]
fn to_account_id_hex(principal: Principal, subaccount: Option<Subaccount>) -> String {
    let sub = subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    AccountIdentifier::new(&principal, &sub).to_hex()
}

// Transfer ICP to a recipient identified by principal + optional subaccount.
// Internally calls AccountIdentifier::new to derive the AccountIdentifier.
// This is the most convenient form when you have a principal.
#[update]
async fn transfer_to_principal(
    amount: Tokens,
    to_principal: Principal,
    to_subaccount: Option<Subaccount>,
) -> Result<BlockIndex, String> {
    let sub = to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    do_transfer(amount, AccountIdentifier::new(&to_principal, &sub)).await
}

// Transfer ICP to a recipient identified by an AccountIdentifier hex string.
// Use this when an exchange or block explorer gives you the destination as a
// 64-char hex string rather than as a principal. The CRC32 checksum embedded
// in the hex is validated before the transfer is attempted.
#[update]
async fn transfer_to_account_id(
    amount: Tokens,
    to_account_id_hex: String,
) -> Result<BlockIndex, String> {
    let account_id = AccountIdentifier::from_hex(&to_account_id_hex)
        .map_err(|e| format!("invalid AccountIdentifier: {}", e))?;
    do_transfer(amount, account_id).await
}

async fn do_transfer(amount: Tokens, to: AccountIdentifier) -> Result<BlockIndex, String> {
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount,
        fee: Tokens::from_e8s(10_000),
        from_subaccount: None,
        to,
        // null: the ledger stores the current IC time as the transaction timestamp.
        // Note: passing null also disables deduplication — if you need protection
        // against duplicate submissions, pass the current time explicitly instead.
        created_at_time: None,
    };
    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, &transfer_args)
        .await
        .map_err(|e| format!("failed to call ledger: {:?}", e))?
        .map_err(|e| format!("ledger transfer error {:?}", e))
}

ic_cdk::export_candid!();
