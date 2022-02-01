use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;

use candid::{candid_method, export_service, CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

mod dip20;
mod types;
use dip20::*;
use types::*;

const ICP_FEE: u64 = 10_000;

#[derive(CandidType, Clone, Deserialize, Serialize, Copy)]
pub struct OrderState {
    id: OrderId,
    owner: Principal,
    from_token_canister_id: Principal,
    from_amount: u128,
    to_token_canister_id: Principal,
    to_amount: u128,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default)]
struct BalancesState(HashMap<Principal, HashMap<Principal, u128>>); // owner -> token_canister_id -> amount
type OrdersState = HashMap<OrderId, OrderState>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, Clone, Deserialize, Serialize, Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
    next_id: OrderId,
    balances: BalancesState,
    orders: OrdersState,
}

impl From<OrderState> for Order {
    fn from(o: OrderState) -> Order {
        Order {
            id: o.id,
            owner: o.owner,
            from: o.from_token_canister_id,
            fromAmount: o.from_amount.into(),
            to: o.to_token_canister_id,
            toAmount: o.to_amount.into(),
        }
    }
}

fn nat_to_u128(n: Nat) -> u128 {
    let n: BigUint = n.try_into().unwrap();
    let n: u128 = n.try_into().unwrap();
    n
}

fn principal_to_subaccount(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}

impl BalancesState {
    fn add_balance(&mut self, owner: &Principal, token_canister_id: &Principal, delta: u128) {
        let balances = self.0.entry(*owner).or_insert_with(HashMap::new);

        if let Some(x) = balances.get_mut(token_canister_id) {
            *x += delta;
        } else {
            balances.insert(*token_canister_id, delta);
        }
    }

    // Returns true on success.
    fn subtract_balance(
        &mut self,
        owner: &Principal,
        token_canister_id: &Principal,
        delta: u128,
    ) -> bool {
        if let Some(balances) = self.0.get_mut(owner) {
            if let Some(x) = balances.get_mut(token_canister_id) {
                *x -= delta;
                if *x == 0 {
                    balances.remove(token_canister_id);
                }
                return true;
            }
        }

        false
    }
}

impl State {
    fn get_balance(&self, token_canister_id: Principal) -> Nat {
        self.balances
            .0
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(0, |v| *v)
            .into()
    }

    fn get_balances(&self) -> Vec<Balance> {
        match self.balances.0.get(&caller()) {
            None => Vec::new(),
            Some(v) => v
                .iter()
                .map(|(token_canister_id, amount)| Balance {
                    owner: caller(),
                    token: *token_canister_id,
                    amount: (*amount).into(),
                })
                .collect(),
        }
    }

    fn get_all_balances(&self) -> Vec<Balance> {
        self.balances
            .0
            .iter()
            .flat_map(|(owner, balances)| {
                balances.iter().map(move |(token, amount)| Balance {
                    owner: *owner,
                    token: *token,
                    amount: (*amount).into(),
                })
            })
            .collect()
    }

    fn get_order(&self, order: OrderId) -> Option<Order> {
        self.orders.get(&order).map(|o| (*o).into())
    }

    fn get_all_orders(&self) -> Vec<Order> {
        self.orders.iter().map(|(_, o)| (*o).into()).collect()
    }

    fn place_order(
        &mut self,
        from_token_canister_id: Principal,
        from_amount: Nat,
        to_token_canister_id: Principal,
        to_amount: Nat,
    ) -> OrderPlacementReceipt {
        ic_cdk::println!("place order");
        let balance = self.get_balance(from_token_canister_id);
        if balance < from_amount {
            return OrderPlacementReceipt::Err(OrderPlacementErr::InvalidOrder);
        }
        let id = self.next_id();
        let from_amount = nat_to_u128(from_amount);
        let to_amount = nat_to_u128(to_amount);
        self.orders.insert(
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
        self.resolve_order(id);

        if let Some(o) = self.orders.get(&id).cloned() {
            OrderPlacementReceipt::Ok(Some(o.into()))
        } else {
            OrderPlacementReceipt::Ok(None)
        }
    }

    fn cancel_order(&mut self, order: OrderId) -> CancelOrderReceipt {
        if let Some(o) = self.orders.get(&order) {
            if o.owner == caller() {
                self.orders.remove(&order);
                CancelOrderReceipt::Ok(order)
            } else {
                CancelOrderReceipt::Err(CancelOrderErr::NotAllowed)
            }
        } else {
            CancelOrderReceipt::Err(CancelOrderErr::NotExistingOrder)
        }
    }

    fn resolve_order(&mut self, id: OrderId) {
        ic_cdk::println!("resolve order");
        let mut matches = Vec::new();
        {
            let a = self.orders.get(&id).unwrap();
            for (order, b) in self.orders.iter() {
                if *order == id {
                    continue;
                }

                if a.from_token_canister_id == b.to_token_canister_id
                    && a.to_token_canister_id == b.from_token_canister_id
                {
                    // Simplified to use multiplication from
                    // (a.from_amount / a.to_amount) * (b.from_amount / b.to_amount) >= 1
                    // which checks that this pair of trades is profitable.
                    if BigUint::from(a.from_amount) * BigUint::from(b.from_amount)
                        >= BigUint::from(a.to_amount) * BigUint::from(b.to_amount)
                    {
                        ic_cdk::println!(
                            "match {}: {} -> {}, {}: {} -> {}",
                            id,
                            a.from_amount,
                            a.to_amount,
                            *order,
                            b.from_amount,
                            b.to_amount
                        );
                        matches.push((id, *order));
                    }
                }
            }
        }
        for m in matches {
            let mut a_to_amount: u128 = 0;
            let mut b_to_amount: u128 = 0;
            {
                if let Some(a) = self.orders.get(&m.0) {
                    if let Some(b) = self.orders.get(&m.1) {
                        // Check if some orders can be completed in their entirety.
                        if b.from_amount >= a.to_amount {
                            a_to_amount = a.to_amount;
                        }
                        if a.from_amount >= b.to_amount {
                            b_to_amount = b.to_amount;
                        }
                        // Check if some orders can be completed partially.
                        if a_to_amount == 0 && b_to_amount > 0 {
                            a_to_amount = b.from_amount;
                            // Verify that we can complete the partial order with natural number tokens remaining.
                            if ((BigUint::from(a_to_amount) * BigUint::from(a.from_amount))
                                % BigUint::from(a.to_amount))
                                != BigUint::from(0u32)
                            {
                                continue;
                            }
                        }
                        if b_to_amount == 0 && a_to_amount > 0 {
                            b_to_amount = a.from_amount;
                            // Verify that we can complete the partial order with natural number tokens remaining.
                            if ((BigUint::from(b_to_amount) * BigUint::from(b.from_amount))
                                % BigUint::from(b.to_amount))
                                != BigUint::from(0u32)
                            {
                                continue;
                            }
                        }
                    }
                }
            }
            if a_to_amount > 0 && b_to_amount > 0 {
                self.process_trade(m.0, m.1, a_to_amount, b_to_amount);
            }
        }
    }

    fn process_trade(&mut self, a: OrderId, b: OrderId, a_to_amount: u128, b_to_amount: u128) {
        ic_cdk::println!("process trade {} {} {} {}", a, b, a_to_amount, b_to_amount);

        let State {
            orders, balances, ..
        } = self;

        let mut order_a = orders.remove(&a).unwrap();
        let mut order_b = orders.remove(&b).unwrap();

        // Calculate "cost" to each
        let a_from_amount: u128 = ((BigUint::from(a_to_amount)
            * BigUint::from(order_a.from_amount))
            / BigUint::from(order_a.to_amount))
        .try_into()
        .unwrap();

        let b_from_amount: u128 = ((BigUint::from(b_to_amount)
            * BigUint::from(order_b.from_amount))
            / BigUint::from(order_b.to_amount))
        .try_into()
        .unwrap();

        // Update order with remaining tokens
        order_a.from_amount -= a_from_amount;
        order_a.to_amount -= a_to_amount;

        order_b.from_amount -= b_from_amount;
        order_b.to_amount -= b_to_amount;

        // Update DEX balances
        balances.subtract_balance(
            &order_a.owner,
            &order_a.from_token_canister_id,
            a_from_amount,
        );
        balances.add_balance(&order_a.owner, &order_a.to_token_canister_id, a_to_amount);

        balances.subtract_balance(
            &order_b.owner,
            &order_b.from_token_canister_id,
            b_from_amount,
        );
        balances.add_balance(&order_b.owner, &order_b.to_token_canister_id, b_to_amount);

        // The DEX keeps any tokens not required to satisfy the parties.
        let dex_amount_a = a_from_amount - b_to_amount;
        if dex_amount_a > 0 {
            balances.add_balance(&ic_cdk::id(), &order_a.from_token_canister_id, dex_amount_a);
        }

        let dex_amount_b = b_from_amount - a_to_amount;
        if dex_amount_b > 0 {
            balances.add_balance(&ic_cdk::id(), &order_b.from_token_canister_id, dex_amount_b);
        }

        // Maintain the order only if not empty
        if order_a.from_amount != 0 {
            orders.insert(order_a.id, order_a);
        }

        if order_b.from_amount != 0 {
            orders.insert(order_b.id, order_b);
        }
    }

    fn next_id(&mut self) -> OrderId {
        self.next_id += 1;
        self.next_id
    }

    // For testing.
    fn credit(&mut self, owner: Principal, token_canister_id: Principal, amount: Nat) {
        ic_cdk::println!("credit {} {}", caller(), self.owner.unwrap());
        assert!(self.owner.unwrap() == caller());
        self.balances
            .add_balance(&owner, &token_canister_id, nat_to_u128(amount));
    }

    // For testing.
    fn clear(&mut self) {
        assert!(self.owner.unwrap() == caller());
        self.orders.clear();
        self.balances.0.clear();
    }
}

#[update]
#[candid_method(update)]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    let caller = caller();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let amount = if token_canister_id == ledger_canister_id {
        deposit_icp(caller).await?
    } else {
        deposit_token(caller, token_canister_id).await?
    };
    STATE.with(|s| {
        s.borrow_mut()
            .balances
            .add_balance(&caller, &token_canister_id, amount)
    });
    DepositReceipt::Ok(amount.into())
}

async fn deposit_icp(caller: Principal) -> Result<u128, DepositErr> {
    let canister_id = ic_cdk::api::id();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

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

    Ok(nat_to_u128(available))
}

#[query(name = "getBalance")]
#[candid_method(query, rename = "getBalance")]
pub fn get_balance(token_canister_id: Principal) -> Nat {
    STATE.with(|s| s.borrow().get_balance(token_canister_id))
}

#[query(name = "getBalances")]
#[candid_method(query, rename = "getBalances")]
pub fn get_balances() -> Vec<Balance> {
    STATE.with(|s| s.borrow().get_balances())
}

#[query(name = "getAllBalances")]
#[candid_method(query, rename = "getAllBalances")]
pub fn get_all_balances() -> Vec<Balance> {
    STATE.with(|s| s.borrow().get_all_balances())
}

#[update(name = "cancelOrder")]
#[candid_method(update, rename = "cancelOrder")]
pub fn cancel_order(order: OrderId) -> CancelOrderReceipt {
    STATE.with(|s| s.borrow_mut().cancel_order(order))
}

#[query(name = "getOrder")]
#[candid_method(query, rename = "getOrder")]
pub fn get_order(order: OrderId) -> Option<Order> {
    STATE.with(|s| s.borrow().get_order(order))
}

#[query(name = "getOrders")]
#[candid_method(query, rename = "getOrders")]
pub fn get_orders() -> Vec<Order> {
    STATE.with(|s| s.borrow().get_all_orders())
}

#[update]
#[candid_method(update)]
pub fn credit(owner: Principal, token_canister_id: Principal, amount: Nat) {
    STATE.with(|s| s.borrow_mut().credit(owner, token_canister_id, amount))
}

#[query(name = "getDepositAddress")]
#[candid_method(query, rename = "getDepositAddress")]
pub fn get_deposit_address() -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());
    AccountIdentifier::new(&canister_id, &subaccount)
}

#[query(name = "getWithdrawalAddress")]
#[candid_method(query, rename = "getWithdrawalAddress")]
pub fn get_withdrawl_address() -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT)
}

#[update(name = "placeOrder")]
#[candid_method(update, rename = "placeOrder")]
pub fn place_order(
    from_token_canister_id: Principal,
    from_amount: Nat,
    to_token_canister_id: Principal,
    to_amount: Nat,
) -> OrderPlacementReceipt {
    STATE.with(|s| {
        s.borrow_mut().place_order(
            from_token_canister_id,
            from_amount,
            to_token_canister_id,
            to_amount,
        )
    })
}

#[update]
#[candid_method(update)]
pub async fn withdraw(
    token_canister_id: Principal,
    amount: Nat,
    address: Principal,
) -> WithdrawReceipt {
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let amount = STATE.with(|s| -> WithdrawReceipt {
        if !s.borrow_mut().balances.subtract_balance(
            &caller(),
            &token_canister_id,
            nat_to_u128(amount.to_owned()),
        ) {
            Err(WithdrawErr::BalanceLow)
        } else {
            Ok(amount)
        }
    })?;

    if token_canister_id == ledger_canister_id {
        let account_id = AccountIdentifier::new(&address, &DEFAULT_SUBACCOUNT);
        withdraw_icp(&amount, account_id).await?;
    } else {
        withdraw_token(token_canister_id, &amount, address).await?;
    };

    WithdrawReceipt::Ok(amount)
}

async fn withdraw_icp(amount: &Nat, account_id: AccountIdentifier) -> Result<Nat, WithdrawErr> {
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: Tokens::from_e8s(nat_to_u128(amount.to_owned()).try_into().unwrap()),
        fee: Tokens::from_e8s(ICP_FEE),
        from_subaccount: Some(DEFAULT_SUBACCOUNT),
        to: account_id,
        created_at_time: None,
    };
    ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)?
        .map_err(|_| WithdrawErr::TransferFailure)?;

    println!("Withdrawal of {} ICP to account {:?}", amount, &account_id);

    Ok(amount.to_owned())
}

async fn withdraw_token(
    token: Principal,
    amount: &Nat,
    address: Principal,
) -> Result<Nat, WithdrawErr> {
    let token = DIP20::new(token);
    let dip_fee = token.get_metadata().await.fee;

    token
        .transfer(address, amount.to_owned() - dip_fee)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)?;

    Ok(amount.to_owned())
}

#[update(name = "getSymbol")]
#[candid_method(update, rename = "getSymbol")]
pub async fn get_symbol(token_canister_id: Principal) -> String {
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

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

#[query(name = "withdrawalAddress")]
#[candid_method(query, rename = "withdrawalAddress")]
pub fn withdrawal_address() -> String {
    AccountIdentifier::new(&caller(), &DEFAULT_SUBACCOUNT).to_string()
}

#[update]
#[candid_method(update)]
pub fn clear() {
    STATE.with(|s| s.borrow_mut().clear())
}

#[init]
#[candid_method(init)]
pub fn init(ledger: Option<Principal>) {
    ic_cdk::setup();
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
        s.borrow_mut().ledger = ledger;
    });
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
