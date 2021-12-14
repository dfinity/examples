use std::cell::RefCell;
use std::hash::Hash;
use candid::{candid_method, CandidType};

use ic_cdk::api::call::call;
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, BlockIndex, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, Memo, Subaccount, Tokens};
use ic_types::Principal;
use serde::{Deserialize, Serialize};

#[cfg(test)] use std::path::PathBuf;
#[cfg(test)] use std::io::Write;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    ledger_canister_id: Principal,
    subaccount: Option<Subaccount>,
    transaction_fee: Tokens
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
            subaccount: None,
            transaction_fee: Tokens::from_e8s(10_000),
        }
    }
}

thread_local! {
    static CONF: RefCell<Conf> = RefCell::new(Conf::default());
}

#[init]
#[candid_method(init)]
fn init(conf: Conf) {
    CONF.with(|c| c.replace(conf));
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct TransferArgs {
    amount: Tokens,
    to_account: Principal,
    to_subaccount: Option<Subaccount>,
}

#[update]
#[candid_method(update)]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!("Transferring {} tokens to account {} subaccount {:?}", &args.amount, &args.to_account, &args.to_subaccount);
    let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    let transfer_args = CONF.with(|conf| {
        let conf = conf.borrow();
        ic_ledger_types::TransferArgs {
            memo: Memo(0),
            amount: args.amount,
            fee: conf.transaction_fee,
            from_subaccount: conf.subaccount,
            to: AccountIdentifier::new(&args.to_account, &to_subaccount),
            created_at_time: None,
        }
    });
    ledger_transfer(&transfer_args).await
}

async fn ledger_transfer(transfer_args: &ic_ledger_types::TransferArgs) -> Result<BlockIndex, String> {
    let ledger_canister_id = CONF.with(|conf| conf.borrow().ledger_canister_id);
    let res: (ic_ledger_types::TransferResult, ) = call(ledger_canister_id, "transfer", (transfer_args, )).await
        .map_err(|e| format!("ledger transfer error {:?}", e))?;
    Ok(res.0.map_err(|e| format!("ledger transfer error {:?}", e))?)
}
