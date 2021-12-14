use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use candid::{candid_method, CandidType};

use ic_cdk::api::call::call;
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, BlockIndex, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, Memo, Subaccount, Tokens};
use ic_types::Principal;
use serde::{Deserialize, Serialize};

#[cfg(test)] use std::path::PathBuf;
#[cfg(test)] use std::io::Write;
use ic_cdk::id;

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

// returns the memo as transaction id
#[update]
#[candid_method(update)]
async fn transfer(args: TransferArgs) -> Result<Memo, String> {
    ic_cdk::println!("Transferring {} tokens to account {} subaccount {:?}", &args.amount, &args.to_account, &args.to_subaccount);
    let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
    let memo = CONF.with(|conf| {
        Memo(hash_transfer_input(&args, conf.borrow().deref()))
    });
    let transfer_args = CONF.with(|conf| {
        let conf = conf.borrow();
        ic_ledger_types::TransferArgs {
            memo,
            amount: args.amount,
            fee: conf.transaction_fee,
            from_subaccount: conf.subaccount,
            to: AccountIdentifier::new(&args.to_account, &to_subaccount),
            created_at_time: None,
        }
    });
    ledger_transfer(&transfer_args).await?;
    Ok(memo)
}

fn hash_transfer_input(args: &TransferArgs, conf: &Conf) -> u64 {
    let mut hasher = DefaultHasher::default();
    std::time::SystemTime::now().hash(&mut hasher);
    id().hash(&mut hasher);
    args.hash(&mut hasher);
    conf.hash(&mut hasher);
    hasher.finish()
}

async fn ledger_transfer(transfer_args: &ic_ledger_types::TransferArgs) -> Result<BlockIndex, String> {
    let ledger_canister_id = CONF.with(|conf| conf.borrow().ledger_canister_id);
    let res: (ic_ledger_types::TransferResult, ) = call(ledger_canister_id, "transfer", (transfer_args, )).await
        .map_err(|e| format!("ledger transfer error {:?}", e))?;
    Ok(res.0.map_err(|e| format!("ledger transfer error {:?}", e))?)
}
