use candid::{
    parser::types::FuncMode,
    types::{Function, Serializer, Type},
    CandidType,
};
use ic_cdk::api::management_canister::http_request::{HttpHeader, HttpMethod, HttpResponse};
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

/// Encapsulating the corresponding candid `func` type.
#[derive(Debug, Clone, Deserialize)]
pub struct TransformFunc(pub candid::Func);

impl CandidType for TransformFunc {
    fn _ty() -> Type {
        Type::Func(Function {
            modes: vec![FuncMode::Query],
            args: vec![TransformArgs::ty()],
            rets: vec![HttpResponse::ty()],
        })
    }

    fn idl_serialize<S: Serializer>(&self, serializer: S) -> Result<(), S::Error> {
        serializer.serialize_function(self.0.principal.as_slice(), &self.0.method)
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct CanisterHttpRequestArgs {
    pub url: String,
    pub max_response_bytes: Option<u64>,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
    pub method: HttpMethod,
    pub transform: Option<TransformContext>,
}
