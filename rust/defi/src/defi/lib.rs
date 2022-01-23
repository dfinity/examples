use candid::{candid_method, export_service, CandidType, Principal};
use ic_cdk::{caller, println};
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(CandidType, Deserialize, Serialize)]
pub struct Conf {
    withdrawl_fee: f32,
    ledger_canister_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct Balance {
    token_canister_id: Principal,
    amount: u128,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Order {
    id: u64,
    owner: Principal,
    from_token_cansiter_id: Principal,
    from_amount: u128,
    to_token_cansiter_id: Principal,
    to_amount: u128,
}

type Orders = HashMap<u64, Order>;
type Balances = HashMap<Principal, Balance>;

pub struct State {
    conf: Conf,
    next_id: u64,
    balances: Balances,
    orders: Orders,
}

export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

impl Default for State {
    fn default() -> Self {
        State {
            conf: Conf {
                ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
                withdrawl_fee: 0.01,
            },
            next_id: 0,
            balances: Balances::new(),
            orders: Orders::new(),
        }
    }
}

impl State {
    fn cancel_order(&mut self, order: u64) -> String {
        if let Some(o) = self.orders.get(&order) {
            if o.owner != caller() {
                return "not owner".to_string();
            }
            self.orders.remove(&order);
            "ok".to_string()
        } else {
            "no found".to_string()
        }
    }

    fn get_order(&self, order: u64) -> Option<Order> {
        self.orders.get(&order).cloned()
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[init]
#[candid_method(init)]
pub fn init(conf: Conf) -> () {
    ic_cdk::setup();
    println!("init!");
    STATE.with(|s| s.borrow_mut().conf = conf);
}

#[query]
#[candid_method(query)]
pub fn cancel_order(order: u64) -> String {
    println!("Canceled {}!", order);
    STATE.with(|s| s.borrow_mut().cancel_order(order))
}

#[query]
#[candid_method(query)]
pub fn get_order(order: u64) -> Option<Order> {
    println!("Get Order {}!", order);
    STATE.with(|s| s.borrow().get_order(order))
}

#[update]
#[candid_method(update)]
pub fn deposit(token_canister_id: Principal, amount: u128) -> () {
    /*
    let canister_id = ic_cdk::api::id();
    let account = AccountIdentifier::new(&canister_id, Subaccount::from(caller()));
    println!(
        "Deposit of {} ICP in account {:?}",
        &amount, &token_canister_id, &account
    );
    let ledger_canister_id = CONF.with(|conf| conf.borrow().ledger_canister_id);
    let transfer_args = CONF.with(|conf| {
        let conf = conf.borrow();
        ic_ledger_types::TransferArgs {
            memo: Memo(0),
            amount: args.amount,
            fee: conf.transaction_fee,
            from_subaccount: conf.subaccount,
            to: AccountIdentifier::new(&args.to_principal, &to_subaccount),
            created_at_time: None,
        }
    });
    ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|e| println!("failed to call ledger: {:?}", e))?
        .map_err(|e| println!("ledger transfer error {:?}", e))
        */
}

#[query]
#[candid_method(query)]
pub fn whoami() -> Principal {
    caller()
}

#[query]
#[candid_method(query)]
pub fn icp_deposit_account() -> String {
    let canister_id = ic_cdk::api::id();
    let subaccount = Subaccount(caller().as_slice().try_into().unwrap());
    let account = AccountIdentifier::new(&canister_id, &subaccount).to_string();
    println!("icp deposit account {}!", account);
    account
}
