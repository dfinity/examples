use std::convert::TryInto;

use candid::{Nat, Principal};
use ic_ledger_types::Subaccount;
use num_bigint::BigUint;
use num_traits::Zero;

pub fn zero() -> Nat {
    Nat(BigUint::zero())
}

pub fn principal_to_subaccount(principal_id: &Principal) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

    Subaccount(subaccount)
}
