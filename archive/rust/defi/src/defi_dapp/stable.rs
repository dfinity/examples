// NOTE: Converting and storing state like this should not be used in production.
// If the state becomes too large, it can prevent future upgrades. This
// is left in as a tool during development. If removed, native types
// can be used throughout, instead.

use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::exchange::{Balances, Exchange};
use crate::types::*;
use crate::{OrderId, State};

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct StableOrder {
    pub id: OrderId,
    pub owner: Principal,
    pub from: Principal,
    pub from_amount: String,
    pub to: Principal,
    pub to_amount: String,
}

impl From<Order> for StableOrder {
    fn from(input: Order) -> Self {
        StableOrder {
            id: input.id,
            owner: input.owner,
            from: input.from,
            from_amount: input.fromAmount.to_string(),
            to: input.to,
            to_amount: input.toAmount.to_string(),
        }
    }
}

impl From<StableOrder> for Order {
    fn from(input: StableOrder) -> Self {
        Order {
            id: input.id,
            owner: input.owner,
            from: input.from,
            fromAmount: input.from_amount.parse().unwrap(),
            to: input.to,
            toAmount: input.to_amount.parse().unwrap(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct StableBalances(pub HashMap<Principal, HashMap<Principal, String>>); // owner -> token_canister_id -> amount

impl From<Balances> for StableBalances {
    fn from(input: Balances) -> Self {
        StableBalances(
            input
                .0
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|(k, v)| (k, v.to_string())).collect()))
                .collect(),
        )
    }
}

impl From<StableBalances> for Balances {
    fn from(input: StableBalances) -> Self {
        Balances(
            input
                .0
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|(k, v)| (k, v.parse().unwrap()))
                            .collect(),
                    )
                })
                .collect(),
        )
    }
}

type StableOrders = HashMap<OrderId, StableOrder>;

#[derive(CandidType, Deserialize, Serialize)]
pub struct StableExchange {
    pub next_id: OrderId,
    pub balances: StableBalances,
    pub orders: StableOrders,
}

impl From<Exchange> for StableExchange {
    fn from(input: Exchange) -> Self {
        StableExchange {
            next_id: input.next_id,
            balances: input.balances.into(),
            orders: input
                .orders
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<StableExchange> for Exchange {
    fn from(input: StableExchange) -> Self {
        Exchange {
            next_id: input.next_id,
            balances: input.balances.into(),
            orders: input
                .orders
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct StableState {
    owner: Option<Principal>,
    ledger: Option<Principal>,
    exchange: StableExchange,
}

impl From<State> for StableState {
    fn from(input: State) -> Self {
        StableState {
            owner: input.owner,
            ledger: input.ledger,
            exchange: input.exchange.into(),
        }
    }
}

impl From<StableState> for State {
    fn from(input: StableState) -> Self {
        State {
            owner: input.owner,
            ledger: input.ledger,
            exchange: input.exchange.into(),
        }
    }
}
