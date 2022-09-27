use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Timestamp = u64;
pub type Rate = f32;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Clone, Debug, PartialEq, CandidType, Serialize, Deserialize)]
pub struct RatesWithInterval {
    pub interval: usize,
    pub rates: HashMap<Timestamp, Rate>,
}
