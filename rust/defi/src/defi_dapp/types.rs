use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone)]
pub struct Order {
    id: u64,
    owner: Principal,
    from_token_canister_id: Principal,
    from_amount: Nat,
    to_token_canister_id: Principal,
    to_amount: Nat,
}

#[derive(CandidType, Clone, Deserialize, Serialize, Copy)]
pub(crate) struct OrderState {
    pub(crate) id: u64,
    pub(crate) owner: Principal,
    pub(crate) from_token_canister_id: Principal,
    pub(crate) from_amount: u128,
    pub(crate) to_token_canister_id: Principal,
    pub(crate) to_amount: u128,
}

impl From<OrderState> for Order {
    fn from(o: OrderState) -> Order {
        Order {
            id: o.id,
            owner: o.owner,
            from_token_canister_id: o.from_token_canister_id,
            from_amount: o.from_amount.into(),
            to_token_canister_id: o.to_token_canister_id,
            to_amount: o.to_amount.into(),
        }
    }
}

pub type CancelOrderReceipt = Result<Nat, CancelOrderErr>;

#[derive(CandidType)]
pub enum CancelOrderErr {
    NotAllowed,
    NotExistingOrder,
}

pub type DepositReceipt = Result<Nat, DepositErr>;

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

pub type OrderPlacementReceipt = Result<Order, OrderPlacementErr>;

#[derive(CandidType)]
pub enum OrderPlacementErr {
    InvalidOrder,
    OrderBookFull,
}

pub type WithdrawReceipt = Result<Nat, WithdrawErr>;

#[derive(CandidType)]
pub enum WithdrawErr {
    BalanceLow,
    TransferFailure,
}
