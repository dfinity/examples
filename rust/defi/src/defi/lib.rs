use candid::{candid_method, export_service, CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Serialize)]
pub struct Conf {
    transaction_fee: f32,
    ledger_canister_id: Principal,
}

export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
            transaction_fee: 0.01,
        }
    }
}

thread_local! {
    static CONF: RefCell<Conf> = RefCell::new(Conf::default());
}

#[candid_method(query)]
pub fn cancel_order(order: Nat) -> () {
    format!("Canceled {}!", order);
}

#[candid_method(query)]
pub fn check_order(order: Nat) -> () {
    format!("Checked {}!", order);
}

#[candid_method(query)]
pub fn deposit(_token_canister_id: Principal, amount: Nat) -> () {
    format!("Deposit {}!", amount);
}

#[candid_method(query)]
pub fn whoami() -> Principal {
    caller()
}
