use ic_cdk::export::{
    candid::{CandidType, Deserialize},
};

#[derive(CandidType, Deserialize)]
pub struct SendRequest {
    pub destination_address: String,
    pub amount_in_satoshi: u64,
}
