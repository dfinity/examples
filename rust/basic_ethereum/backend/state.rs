use crate::ecdsa::EcdsaPublicKey;
use crate::{EthereumNetwork, InitArg};
use evm_rpc_types::{EthMainnetService, EthSepoliaService, RpcServices};
use ic_cdk_management_canister::{EcdsaCurve, EcdsaKeyId};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

pub fn init_state(init_arg: InitArg) {
    STATE.with(|s| *s.borrow_mut() = State::from(init_arg));
}

pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|s| f(s.borrow().deref()))
}

pub fn mutate_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|s| f(s.borrow_mut().deref_mut()))
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    ethereum_network: EthereumNetwork,
    ecdsa_key_name: String,
    ecdsa_public_key: Option<EcdsaPublicKey>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ethereum_network: EthereumNetwork::default(),
            ecdsa_key_name: "test_key_1".to_string(),
            ecdsa_public_key: None,
        }
    }
}

impl State {
    pub fn ecdsa_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: self.ecdsa_key_name.clone(),
        }
    }

    pub fn ethereum_network(&self) -> EthereumNetwork {
        self.ethereum_network
    }

    // Returns the RPC services to use for multi-provider calls.
    // Uses PublicNode by default (no API key required) so the example works
    // out of the box locally and without credentials.
    //
    // For production, pass `None` to use all configured providers (including
    // API-key-based ones like Alchemy/Ankr), or add multiple providers for
    // better consensus. API keys are configured via the EVM RPC canister's
    // `authorize` and `updateProvider` endpoints — see README for details.
    pub fn evm_rpc_services(&self) -> RpcServices {
        match self.ethereum_network {
            EthereumNetwork::Mainnet => {
                RpcServices::EthMainnet(Some(vec![EthMainnetService::PublicNode]))
            }
            EthereumNetwork::Sepolia => {
                RpcServices::EthSepolia(Some(vec![EthSepoliaService::PublicNode]))
            }
        }
    }

    pub fn single_evm_rpc_service(&self) -> RpcServices {
        match self.ethereum_network {
            EthereumNetwork::Mainnet => {
                RpcServices::EthMainnet(Some(vec![EthMainnetService::PublicNode]))
            }
            EthereumNetwork::Sepolia => {
                RpcServices::EthSepolia(Some(vec![EthSepoliaService::PublicNode]))
            }
        }
    }
}

impl From<InitArg> for State {
    fn from(init_arg: InitArg) -> Self {
        State {
            ethereum_network: init_arg.ethereum_network.unwrap_or_default(),
            ecdsa_key_name: init_arg.ecdsa_key_name.unwrap_or_else(|| "test_key_1".to_string()),
            ecdsa_public_key: None,
        }
    }
}

pub async fn lazy_call_ecdsa_public_key() -> EcdsaPublicKey {
    use ic_cdk_management_canister::{ecdsa_public_key, EcdsaPublicKeyArgs as EcdsaPublicKeyArgument};

    if let Some(ecdsa_pk) = read_state(|s| s.ecdsa_public_key.clone()) {
        return ecdsa_pk;
    }
    let key_id = read_state(|s| s.ecdsa_key_id());
    let key_name = key_id.name.clone();
    let response = ecdsa_public_key(&EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![],
        key_id,
    })
    .await
    .unwrap_or_else(|e| {
        ic_cdk::trap(&format!(
            "failed to get ECDSA public key for key '{}': {:?}",
            key_name, e,
        ))
    });
    let pk = EcdsaPublicKey::from(response);
    mutate_state(|s| s.ecdsa_public_key = Some(pk.clone()));
    pk
}
