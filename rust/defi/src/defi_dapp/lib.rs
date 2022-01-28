use candid::{candid_method, export_service, CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;

mod dip20;
mod types;
use dip20::*;
use types::*;

const ICP_FEE: u64 = 10_000;

type OrdersState = HashMap<u64, OrderState>;
type BalancesState = HashMap<Principal, HashMap<Principal, u128>>; // owner -> token_canister_id -> amount

#[derive(CandidType, Clone, Deserialize, Serialize, Default)]
pub struct State {
    next_id: u64,
    balances: BalancesState,
    orders: OrdersState,
}

impl State {
    fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[query]
#[candid_method(query)]
pub fn balance(token_canister_id: Principal) -> Nat {
    STATE.with(|s| {
        let state = s.borrow();

        state
            .balances
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(0, |v| *v)
            .into()
    })
}

#[query]
#[candid_method(query)]
pub async fn balances() -> Vec<(String, Nat)> {
    // Collect all balances into a Vec
    let balances = STATE.with(|s| {
        let state = s.borrow();

        if let Some(balances) = state.balances.get(&caller()) {
            balances
                .iter()
                .map(|(token, balance)| (token.clone(), balance.clone()))
                .collect()
        } else {
            Vec::new()
        }
    });

    // Transform the Vec<Token, balance> into Vec<Symbol, balance>
    let mut out = Vec::new();
    for (token, balance) in balances.into_iter() {
        out.push((symbol(token).await, balance.into()));
    }

    out
}

#[update]
#[candid_method(update)]
pub fn cancel_order(order: u64) -> CancelOrderReceipt {
    STATE.with(|s| {
        let mut state = s.borrow_mut();

        if let Some(o) = state.orders.get(&order) {
            if o.owner != caller() {
                CancelOrderReceipt::Err(CancelOrderErr::NotAllowed)
            } else {
                state.orders.remove(&order);

                CancelOrderReceipt::Ok(order.into())
            }
        } else {
            CancelOrderReceipt::Err(CancelOrderErr::NotExistingOrder)
        }
    })
}

#[query]
#[candid_method(query)]
fn check_order(order: u64) -> Option<Order> {
    STATE.with(|s| {
        let state = s.borrow();

        state.orders.get(&order).map(|v| (*v).into())
    })
}

#[update]
#[candid_method(update)]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    let amount =
        if token_canister_id == Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap() {
            deposit_icp(caller()).await?
        } else {
            deposit_token(caller(), token_canister_id).await?
        };

    STATE.with(|s| {
        let mut state = s.borrow_mut();

        let balances = state
            .balances
            .entry(caller())
            .or_insert_with(|| HashMap::new());

        add_balance(balances, &token_canister_id, amount);
    });

    DepositReceipt::Ok(amount.into())
}

async fn deposit_icp(caller: Principal) -> Result<u128, DepositErr> {
    let canister_id = ic_cdk::api::id();
    // let ledger_canister_id = MAINNET_LEDGER_CANISTER_ID;
    let ledger_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));

    let balance_args = ic_ledger_types::AccountBalanceArgs { account };
    let balance = ic_ledger_types::account_balance(ledger_canister_id, balance_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    if balance.e8s() < 2 * ICP_FEE {
        return Err(DepositErr::BalanceLow);
    }

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: balance - Tokens::from_e8s(ICP_FEE),
        fee: Tokens::from_e8s(ICP_FEE),
        from_subaccount: Some(principal_to_subaccount(&caller)),
        to: AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT),
        created_at_time: None,
    };
    ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?
        .map_err(|_| DepositErr::TransferFailure)?;

    println!(
        "Deposit of {} ICP in account {:?}",
        balance - Tokens::from_e8s(2 * ICP_FEE),
        &account
    );

    Ok((balance.e8s() - 2 * ICP_FEE).into())
}

async fn deposit_token(caller: Principal, token: Principal) -> Result<u128, DepositErr> {
    let token = DIP20::new(token);
    let dip_fee = token.get_metadata().await.fee;

    let allowance = token.allowance(caller, ic_cdk::api::id()).await;

    let available = allowance - dip_fee;

    token
        .transfer_from(caller, ic_cdk::api::id(), available.to_owned())
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    Ok(nat_to_u128(available).try_into().unwrap())
}

#[query]
#[candid_method(query)]
pub fn deposit_address() -> String {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount).to_string()
}

#[query]
#[candid_method(query)]
pub fn list_order() -> Vec<Order> {
    STATE.with(|s| {
        let state = s.borrow();

        state.orders.iter().map(|(_, o)| (*o).into()).collect()
    })
}

#[update]
#[candid_method(update)]
pub fn place_order(
    from_token_canister_id: Principal,
    from_amount: Nat,
    to_token_canister_id: Principal,
    to_amount: Nat,
) -> OrderPlacementReceipt {
    let balance = balance(from_token_canister_id);
    if balance < from_amount {
        return OrderPlacementReceipt::Err(OrderPlacementErr::InvalidOrder);
    }

    let from_amount = nat_to_u128(from_amount);
    let to_amount = nat_to_u128(to_amount);

    let order = STATE.with(|s| {
        let mut state = s.borrow_mut();

        let id = state.next_id();
        let order = OrderState {
            id,
            owner: caller(),
            from_token_canister_id,
            from_amount,
            to_token_canister_id,
            to_amount,
        };

        state.orders.insert(
            id,
            OrderState {
                id,
                owner: caller(),
                from_token_canister_id,
                from_amount,
                to_token_canister_id,
                to_amount,
            },
        );

        order
    });

    resolve_orders(order.id);

    OrderPlacementReceipt::Ok(order.into())
}

#[update]
#[candid_method(update)]
pub async fn withdraw(token_canister_id: Principal, amount: Nat) -> WithdrawReceipt {
    let amount = STATE.with(|s| -> WithdrawReceipt {
        let mut state = s.borrow_mut();

        let balances = state
            .balances
            .entry(caller())
            .or_insert_with(|| HashMap::new());

        let balance = balances.get(&token_canister_id).map(|v| *v).unwrap_or(0);

        if amount < balance {
            return Err(WithdrawErr::BalanceLow);
        }

        subtract_balance(balances, &token_canister_id, nat_to_u128(amount.to_owned()));

        Ok(amount)
    })?;

    if token_canister_id == Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap() {
        withdraw_icp(caller(), &amount).await?;
    } else {
        withdraw_token(caller(), token_canister_id, &amount).await?;
    };

    WithdrawReceipt::Ok(amount)
}

async fn withdraw_icp(caller: Principal, amount: &Nat) -> Result<Nat, WithdrawErr> {
    // let ledger_canister_id = MAINNET_LEDGER_CANISTER_ID;
    let ledger_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(nat_to_u128(amount.to_owned()).try_into().unwrap()),
        fee: Tokens::from_e8s(ICP_FEE),
        from_subaccount: Some(DEFAULT_SUBACCOUNT),
        to: AccountIdentifier::new(&caller, &DEFAULT_SUBACCOUNT),
        created_at_time: None,
    };
    ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)?
        .map_err(|_| WithdrawErr::TransferFailure)?;

    println!("Withdrawal of {} ICP to account {:?}", amount, &caller);

    Ok(amount.to_owned())
}

async fn withdraw_token(
    caller: Principal,
    token: Principal,
    amount: &Nat,
) -> Result<Nat, WithdrawErr> {
    let token = DIP20::new(token);
    let dip_fee = token.get_metadata().await.fee;

    token
        .transfer(caller, amount.to_owned() - dip_fee)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)?;

    Ok(amount.to_owned())
}

#[update]
#[candid_method(update)]
pub async fn symbol(token_canister_id: Principal) -> String {
    // let ledger_canister_id = MAINNET_LEDGER_CANISTER_ID;
    let ledger_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    if token_canister_id == ledger_canister_id {
        "ICP".to_string()
    } else {
        DIP20::new(token_canister_id).get_metadata().await.symbol
    }
}

#[query]
#[candid_method(query)]
pub fn whoami() -> Principal {
    caller()
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable_state = STATE.with(|s| s.take());
    ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let (stable_state,) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");
    STATE.with(|s| {
        s.replace(stable_state);
    });
}

export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

// ----------------------------------------------------------------------------

fn add_balance(m: &mut HashMap<Principal, u128>, token_canister_id: &Principal, delta: u128) {
    if let Some(x) = m.get_mut(&token_canister_id) {
        *x += delta;
    } else {
        m.insert(*token_canister_id, delta);
    }
}

fn subtract_balance(m: &mut HashMap<Principal, u128>, token_canister_id: &Principal, delta: u128) {
    let x = m.get_mut(&token_canister_id).unwrap();
    *x -= delta;
    if *x == 0 {
        m.remove(token_canister_id);
    }
}

fn nat_to_u128(n: Nat) -> u128 {
    let n: BigUint = n.try_into().unwrap();
    let n: u128 = n.try_into().unwrap();
    n
}

fn resolve_orders(id: u64) {
    STATE.with(|s| {
        let state = s.borrow();

        let mut matches = Vec::new();
        {
            let a = state.orders.get(&id).unwrap();
            for (order, b) in state.orders.iter() {
                if *order == id {
                    continue;
                }

                if a.from_token_canister_id == b.to_token_canister_id
                    && a.to_token_canister_id == b.from_token_canister_id
                {
                    let a_ratio = a.from_amount / a.to_amount;
                    let b_ratio = b.to_amount / b.from_amount;

                    if a_ratio == b_ratio {
                        matches.push((id, *order));
                    }
                }
            }
        }

        for m in matches {
            let mut amount = 0;
            {
                if let Some(a) = state.orders.get(&m.0) {
                    if let Some(b) = state.orders.get(&m.1) {
                        amount = std::cmp::min(a.from_amount, b.to_amount);
                    }
                }
            }
            if amount > 0 {
                process_trade(m, amount);
            }
        }
    })
}

fn process_trade(pair: (u64, u64), amount: u128) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();

        let State {
            orders, balances, ..
        } = &mut *state;

        let mut order_1 = orders.remove(&pair.0).unwrap();
        let mut order_2 = orders.remove(&pair.1).unwrap();
        let token_a = amount;
        let token_b = amount * (order_1.to_amount / order_1.from_amount);

        // Remove traded tokens from the order
        order_1.from_amount -= token_a;
        order_1.to_amount -= token_b;

        order_2.to_amount -= token_b;
        order_2.from_amount -= token_a;

        // Update balance book

        let o = balances.get_mut(&order_1.owner.clone()).unwrap();
        subtract_balance(o, &order_1.from_token_canister_id, amount);
        add_balance(o, &order_1.to_token_canister_id, amount);

        let o = balances.get_mut(&order_2.owner.clone()).unwrap();
        add_balance(o, &order_2.from_token_canister_id, amount);
        subtract_balance(o, &order_2.to_token_canister_id, amount);

        // Add back any orders that are not empty
        if order_1.from_amount != 0 {
            orders.insert(pair.0, order_1);
        }

        if order_2.to_amount != 0 {
            orders.insert(pair.1, order_2);
        }
    })
}

fn principal_to_subaccount(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}
