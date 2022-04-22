use std::collections::HashMap;
use std::convert::TryInto;

use candid::{CandidType, Nat, Principal};
use ic_cdk::caller;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::types::*;
use crate::utils::nat_to_u128;
use crate::OrderId;

#[derive(CandidType, Clone, Deserialize, Serialize, Copy)]
pub struct OrderState {
    pub id: OrderId,
    pub owner: Principal,
    pub from_token_canister_id: Principal,
    pub from_amount: u128,
    pub to_token_canister_id: Principal,
    pub to_amount: u128,
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

#[derive(CandidType, Clone, Deserialize, Serialize, Default)]
pub struct BalancesState(pub HashMap<Principal, HashMap<Principal, u128>>); // owner -> token_canister_id -> amount
type OrdersState = HashMap<OrderId, OrderState>;

#[derive(CandidType, Clone, Deserialize, Serialize, Default)]
pub struct Exchange {
    pub next_id: OrderId,
    pub balances: BalancesState,
    pub orders: OrdersState,
}

impl BalancesState {
    #[must_use]
    pub fn add_balance(
        &mut self,
        owner: &Principal,
        token_canister_id: &Principal,
        delta: u128,
    ) -> bool {
        let balances = self.0.entry(*owner).or_insert_with(HashMap::new);

        if let Some(x) = balances.get_mut(token_canister_id) {
            if let Some(v) = x.checked_add(delta) {
                *x = v;
            } else {
                return false;
            }
        } else {
            balances.insert(*token_canister_id, delta);
        }

        true
    }

    // Tries to substract balance from user account. Checks for overflows
    pub fn subtract_balance(
        &mut self,
        owner: &Principal,
        token_canister_id: &Principal,
        delta: u128,
    ) -> bool {
        if let Some(balances) = self.0.get_mut(owner) {
            if let Some(x) = balances.get_mut(token_canister_id) {
                match (*x).checked_sub(delta) {
                    Some(num) => {
                        *x = num;
                    }
                    None => return false,
                }
                // no need to keep an empty token record
                if *x == 0 {
                    balances.remove(token_canister_id);
                }
                return true;
            }
        }

        false
    }
}

impl Exchange {
    pub fn get_balance(&self, token_canister_id: Principal) -> Nat {
        self.balances
            .0
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(0, |v| *v)
            .into()
    }

    pub fn get_balances(&self) -> Vec<Balance> {
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

    pub fn get_all_balances(&self) -> Vec<Balance> {
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

    pub fn get_order(&self, order: OrderId) -> Option<Order> {
        self.orders.get(&order).map(|o| (*o).into())
    }

    pub fn get_all_orders(&self) -> Vec<Order> {
        self.orders.iter().map(|(_, o)| (*o).into()).collect()
    }

    pub fn place_order(
        &mut self,
        from_token_canister_id: Principal,
        from_amount: Nat,
        to_token_canister_id: Principal,
        to_amount: Nat,
    ) -> OrderPlacementReceipt {
        ic_cdk::println!("place order");
        if from_amount <= 0u8 || to_amount <= 0u8 {
            return OrderPlacementReceipt::Err(OrderPlacementErr::InvalidOrder);
        }

        if self.check_for_sell_orders(from_token_canister_id) {
            return OrderPlacementReceipt::Err(OrderPlacementErr::InvalidOrder);
        }

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
        self.resolve_order(id)?;

        if let Some(o) = self.orders.get(&id) {
            OrderPlacementReceipt::Ok(Some((*o).into()))
        } else {
            OrderPlacementReceipt::Ok(None)
        }
    }

    pub fn check_for_sell_orders(&self, from_token_canister_id: Principal) -> bool {
        self.orders
            .values()
            .find(|v| (v.from_token_canister_id == from_token_canister_id) && (v.owner == caller()))
            .is_some()
    }

    pub fn cancel_order(&mut self, order: OrderId) -> CancelOrderReceipt {
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

    fn resolve_order(&mut self, id: OrderId) -> Result<(), OrderPlacementErr> {
        ic_cdk::println!("resolve order");
        let mut matches = Vec::new();
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
                    matches.push((a.to_owned(), b.to_owned()));
                }
            }
        }
        for m in matches {
            let mut a_to_amount: u128 = 0;
            let mut b_to_amount: u128 = 0;
            let (a, b) = m;
            // Check if some orders can be completed in their entirety.
            if b.from_amount >= a.to_amount {
                a_to_amount = a.to_amount;
            }
            if a.from_amount >= b.to_amount {
                b_to_amount = b.to_amount;
            }
            // Check if some orders can be completed partially.
            if check_orders(a, b, &mut a_to_amount, b_to_amount) {
                continue;
            }
            if check_orders(b, a, &mut b_to_amount, a_to_amount) {
                continue;
            }

            if a_to_amount > 0 && b_to_amount > 0 {
                self.process_trade(a.id, b.id, a_to_amount, b_to_amount)?;
            }
        }

        Ok(())
    }

    fn process_trade(
        &mut self,
        a: OrderId,
        b: OrderId,
        a_to_amount: u128,
        b_to_amount: u128,
    ) -> Result<(), OrderPlacementErr> {
        ic_cdk::println!("process trade {} {} {} {}", a, b, a_to_amount, b_to_amount);

        let Exchange {
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
        if !balances.add_balance(&order_a.owner, &order_a.to_token_canister_id, a_to_amount) {
            return Err(OrderPlacementErr::IntegerOverflow);
        }

        balances.subtract_balance(
            &order_b.owner,
            &order_b.from_token_canister_id,
            b_from_amount,
        );
        if !balances.add_balance(&order_b.owner, &order_b.to_token_canister_id, b_to_amount) {
            return Err(OrderPlacementErr::IntegerOverflow);
        }

        // The DEX keeps any tokens not required to satisfy the parties.
        let dex_amount_a = a_from_amount - b_to_amount;
        if dex_amount_a > 0 {
            if !balances.add_balance(&ic_cdk::id(), &order_a.from_token_canister_id, dex_amount_a) {
                return Err(OrderPlacementErr::IntegerOverflow);
            }
        }

        let dex_amount_b = b_from_amount - a_to_amount;
        if dex_amount_b > 0 {
            if !balances.add_balance(&ic_cdk::id(), &order_b.from_token_canister_id, dex_amount_b) {
                return Err(OrderPlacementErr::IntegerOverflow);
            }
        }

        // Maintain the order only if not empty
        if order_a.from_amount != 0 {
            orders.insert(order_a.id, order_a);
        }

        if order_b.from_amount != 0 {
            orders.insert(order_b.id, order_b);
        }

        Ok(())
    }

    fn next_id(&mut self) -> OrderId {
        self.next_id += 1;
        self.next_id
    }
}

fn check_orders(
    first: OrderState,
    second: OrderState,
    first_to_amount: &mut u128,
    second_to_amount: u128,
) -> bool {
    if *first_to_amount == 0 && second_to_amount > 0 {
        *first_to_amount = second.from_amount;
        // Verify that we can complete the partial order with natural number tokens remaining.
        if ((BigUint::from(*first_to_amount) * BigUint::from(first.from_amount))
            % BigUint::from(first.to_amount))
            != BigUint::from(0u32)
        {
            return true;
        }
    }

    false
}
