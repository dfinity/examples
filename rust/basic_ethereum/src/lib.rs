use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId};
use ic_cdk::{init, query};
use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<InitArg> = RefCell::default();
}

#[init]
pub fn init(maybe_init: Option<InitArg>) {
    if let Some(InitArg {
        ethereum_network,
        ecdsa_key_name,
    }) = maybe_init
    {
        STATE.with(|state| {
            *state.borrow_mut() = InitArg {
                ethereum_network,
                ecdsa_key_name,
            };
        });
    }
}

#[query]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct InitArg {
    pub ethereum_network: EthereumNetwork,
    pub ecdsa_key_name: EcdsaKeyName,
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq)]
pub enum EthereumNetwork {
    Mainnet,
    #[default]
    Sepolia,
}

impl EthereumNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            EthereumNetwork::Mainnet => 1,
            EthereumNetwork::Sepolia => 11155111,
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq)]
pub enum EcdsaKeyName {
    #[default]
    TestKeyLocalDevelopment,
    TestKey1,
    ProductionKey1,
}

impl From<EcdsaKeyName> for EcdsaKeyId {
    fn from(value: EcdsaKeyName) -> Self {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match value {
                EcdsaKeyName::TestKeyLocalDevelopment => "dfx_test_key",
                EcdsaKeyName::TestKey1 => "test_key_1",
                EcdsaKeyName::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}
