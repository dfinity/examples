use candid::{candid_method, export_service, CandidType, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{AccountIdentifier, Subaccount, MAINNET_LEDGER_CANISTER_ID};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(CandidType)]
pub enum CancelOrderReceipt {
    Ok(OrderId),
    Err(CancelOrderErr),
}

#[derive(CandidType)]
pub enum CancelOrderErr {
    NotAllowed,
    NotExistingOrder,
}

#[derive(CandidType)]
pub enum DepositReceipt {
    Ok(Nat),
    Err(DepositErr),
}

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

#[derive(CandidType)]
pub enum OrderPlacementReceipt {
    Ok(Order),
    Executed,
    Err(OrderPlacementErr),
}

#[derive(CandidType)]
pub enum OrderPlacementErr {
    InvalidOrder,
    OrderBookFull,
}

#[derive(CandidType)]
pub enum WithdrawReceipt {
    Ok(Nat),
    Err(WithdrawErr),
}

#[derive(CandidType)]
pub enum WithdrawErr {
    BalanceLow,
    TransferFailure,
}

#[derive(CandidType)]
pub struct Balance {
    owner: Principal,
    token: Principal,
    amount: Nat,
}

type OrderId = u32;

#[derive(CandidType, Clone)]
#[allow(non_snake_case)]
pub struct Order {
    id: OrderId,
    owner: Principal,
    from: Principal,
    fromAmount: Nat,
    to: Principal,
    toAmount: Nat,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Copy)]
pub struct OrderState {
    id: OrderId,
    owner: Principal,
    from_token_canister_id: Principal,
    from_amount: u128,
    to_token_canister_id: Principal,
    to_amount: u128,
}

type OrdersState = HashMap<OrderId, OrderState>;
type BalancesState = HashMap<Principal, HashMap<Principal, u128>>; // owner -> token_canister_id -> amount

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct State {
    owner: Option<Principal>,
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

impl Default for State {
    fn default() -> Self {
        State {
            owner: None,
            next_id: 0,
            balances: BalancesState::new(),
            orders: OrdersState::new(),
        }
    }
}

impl State {
    fn add_balance(&mut self, owner: &Principal, token_canister_id: &Principal, delta: u128) {
        if !self.balances.contains_key(&owner) {
            self.balances.insert(*owner, HashMap::new());
        }
        let m = self.balances.get_mut(&owner).unwrap();
        if let Some(x) = m.get_mut(&token_canister_id) {
            *x += delta;
        } else {
            m.insert(*token_canister_id, delta);
        }
    }

    fn subtract_balance(&mut self, owner: &Principal, token_canister_id: &Principal, delta: u128) {
        let m = self.balances.get_mut(owner).unwrap();
        let x = m.get_mut(&token_canister_id).unwrap();
        *x -= delta;
        if *x == 0 {
            m.remove(token_canister_id);
        }
    }

    fn get_balance(&self, token_canister_id: Principal) -> Nat {
        self.balances
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(0, |v| *v)
            .into()
    }

    fn get_balances(&self) -> Vec<Balance> {
        match self.balances.get(&caller()) {
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
        let mut result: Vec<Balance> = Vec::new();
        for (owner, v) in self.balances.iter() {
            for (token_canister_id, amount) in v.iter() {
                result.push(Balance {
                    owner: *owner,
                    token: *token_canister_id,
                    amount: (*amount).into(),
                });
            }
        }
        result
    }

    fn deposit(&mut self, token_canister_id: Principal, amount: Nat) -> DepositReceipt {
        // let amount = 10;

        self.add_balance(&caller(), &token_canister_id, nat_to_u128(amount.clone()));
        DepositReceipt::Ok(amount.into())

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

    fn get_order(&self, order: OrderId) -> Option<Order> {
        match self.orders.get(&order) {
            None => None,
            Some(o) => Some((*o).into()),
        }
    }

    fn get_all_orders(&self) -> Vec<Order> {
        self.orders.iter().map(|(_, o)| (*o).into()).collect()
    }

    fn next_id(&mut self) -> OrderId {
        self.next_id += 1;
        self.next_id
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
                from_amount: from_amount,
                to_token_canister_id,
                to_amount: to_amount.try_into().unwrap(),
            },
        );
        self.resolve_order(id);

        if let Some(order) = self.orders.get(&id).cloned() {
            OrderPlacementReceipt::Ok(order.into())
        } else {
            OrderPlacementReceipt::Executed
        }
    }

    fn cancel_order(&mut self, order: OrderId) -> CancelOrderReceipt {
        if let Some(o) = self.orders.get(&order) {
            if o.owner == caller() {
                CancelOrderReceipt::Err(CancelOrderErr::NotAllowed)
            } else {
                self.orders.remove(&order);

                CancelOrderReceipt::Ok(order)
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
                        if b.from_amount > a.to_amount {
                            a_to_amount = a.to_amount;
                        }
                        if a.from_amount > b.to_amount {
                            b_to_amount = b.to_amount;
                        }
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
        let remove_a;
        let remove_b;
        {
            {
                let a_clone;
                let a_from_amount: u128;
                {
                    // Update a side.
                    let a = self.orders.get_mut(&a).unwrap();
                    a_from_amount = ((BigUint::from(a_to_amount) * BigUint::from(a.from_amount))
                        / BigUint::from(a.to_amount))
                    .try_into()
                    .unwrap();
                    a.from_amount -= a_from_amount;
                    a.to_amount -= a_to_amount;
                    remove_a = a.from_amount == 0;
                    a_clone = a.clone();
                }

                self.subtract_balance(
                    &a_clone.owner,
                    &a_clone.from_token_canister_id,
                    a_from_amount,
                );
                self.add_balance(&a_clone.owner, &a_clone.to_token_canister_id, a_to_amount);
                // The DEX keeps any tokens not required to satisfy the parties.
                let dex_amount = a_from_amount - b_to_amount;
                if dex_amount > 0 {
                    self.add_balance(&ic_cdk::id(), &a_clone.from_token_canister_id, dex_amount);
                }
            }

            {
                let b_clone;
                let b_from_amount: u128;
                {
                    // Update b side.
                    let b = self.orders.get_mut(&b).unwrap();
                    b_from_amount = ((BigUint::from(b_to_amount) * BigUint::from(b.from_amount))
                        / BigUint::from(b.to_amount))
                    .try_into()
                    .unwrap();
                    b.from_amount -= b_from_amount;
                    b.to_amount -= b_to_amount;
                    remove_b = b.to_amount == 0;
                    b_clone = b.clone();
                }

                self.subtract_balance(
                    &b_clone.owner,
                    &b_clone.from_token_canister_id,
                    b_from_amount,
                );
                self.add_balance(&b_clone.owner, &b_clone.to_token_canister_id, b_to_amount);
                // The DEX keeps any tokens not required to satisfy the parties.
                let dex_amount = b_from_amount - a_to_amount;
                if dex_amount > 0 {
                    self.add_balance(&ic_cdk::id(), &b_clone.from_token_canister_id, dex_amount);
                }
            }
        }
        if remove_a {
            self.orders.remove(&a);
        }
        if remove_b {
            self.orders.remove(&b);
        }
    }

    fn withdraw(&mut self, token_canister_id: Principal, amount: Nat) -> WithdrawReceipt {
        let amount = nat_to_u128(amount);
        let o = self.balances.get_mut(&caller()).unwrap();
        let left;
        if let Some(b) = o.get(&token_canister_id) {
            if b < &amount {
                return WithdrawReceipt::Err(WithdrawErr::TransferFailure);
            }
            left = b - amount;
        } else {
            return WithdrawReceipt::Err(WithdrawErr::BalanceLow);
        }
        self.subtract_balance(&caller(), &token_canister_id, amount);
        WithdrawReceipt::Ok(left.into())
    }

    fn clear(&mut self) -> String {
        if let Some(owner) = self.owner {
            if owner != caller() {
                return "not authorized".into();
            }
        } else {
            return "not initialized".into();
        }
        self.orders.clear();
        self.balances.clear();
        "ok".into()
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn principal_to_subaccount(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}

#[init]
#[candid_method(init)]
pub fn init() -> () {
    ic_cdk::setup();
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
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

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getBalance(token_canister_id: Principal) -> Nat {
    STATE.with(|s| s.borrow().get_balance(token_canister_id))
}

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getBalances() -> Vec<Balance> {
    STATE.with(|s| s.borrow().get_balances())
}

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getAllBalances() -> Vec<Balance> {
    STATE.with(|s| s.borrow().get_all_balances())
}

#[update]
#[candid_method(update)]
#[allow(non_snake_case)]
pub fn cancelOrder(order: OrderId) -> CancelOrderReceipt {
    STATE.with(|s| s.borrow_mut().cancel_order(order))
}

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getOrder(order: OrderId) -> Option<Order> {
    STATE.with(|s| s.borrow().get_order(order))
}

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getOrders() -> Vec<Order> {
    STATE.with(|s| s.borrow().get_all_orders())
}

#[update]
#[candid_method(update)]
#[allow(non_snake_case)]
pub fn deposit(token_canister_id: Principal, amount: Nat) -> DepositReceipt {
    STATE.with(|s| s.borrow_mut().deposit(token_canister_id, amount))
}

#[query]
#[candid_method(query)]
#[allow(non_snake_case)]
pub fn getDepositAddress() -> String {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());
    let account = AccountIdentifier::new(&canister_id, &subaccount).to_string();
    account
}

#[update]
#[candid_method(update)]
#[allow(non_snake_case)]
pub fn placeOrder(
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
#[allow(non_snake_case)]
pub fn getSymbol(_token_canister_id: Principal) -> String {
    "XXX".to_string()
}

#[query]
#[candid_method(query)]
pub fn whoami() -> Principal {
    caller()
}

#[update]
#[candid_method(update)]
pub fn withdraw(token_canister_id: Principal, amount: Nat) -> WithdrawReceipt {
    STATE.with(|s| s.borrow_mut().withdraw(token_canister_id, amount))
}

#[update]
#[candid_method(update)]
pub fn clear() -> String {
    STATE.with(|s| s.borrow_mut().clear())
}

export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
