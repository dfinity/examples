use candid::{CandidType, Deserialize, Nat, Principal};
use evm_rpc_canister_types::{
    BlockTag, EvmRpcCanister, GetTransactionCountArgs, GetTransactionCountResult,
    MultiGetTransactionCountResult, RpcServices,
};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyResponse};
use ic_cdk::{init, update};
use ic_crypto_ecdsa_secp256k1::PublicKey;
use ic_ethereum_types::Address;
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

pub const CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai
pub const EVM_RPC: EvmRpcCanister = EvmRpcCanister(CANISTER_ID);

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[init]
pub fn init(maybe_init: Option<InitArg>) {
    if let Some(init_arg) = maybe_init {
        STATE.with(|state| {
            *state.borrow_mut() = State::from(init_arg);
        });
    }
}

#[update]
pub async fn ethereum_address(owner: Option<Principal>) -> String {
    let caller = validate_caller_not_anonymous();
    let owner = owner.unwrap_or(caller);
    let address = Address::from(&lazy_call_ecdsa_public_key().await.derive(&owner));
    address.to_string()
}

#[update]
pub async fn transaction_count(owner: Option<Principal>, block: Option<BlockTag>) -> Nat {
    let caller = validate_caller_not_anonymous();
    let owner = owner.unwrap_or(caller);
    let address = Address::from(&lazy_call_ecdsa_public_key().await.derive(&owner));
    let rpc_services = read_state(|s| s.evm_rpc_services());
    let args = GetTransactionCountArgs {
        address: address.to_string(),
        block: block.unwrap_or(BlockTag::Finalized),
    };
    let (result,) = EVM_RPC
        .eth_get_transaction_count(rpc_services, None, args.clone(), 2_000_000_000_u128)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "failed to get transaction count for {:?}, error: {:?}",
                args, e
            )
        });
    match result {
        MultiGetTransactionCountResult::Consistent(consistent_result) => match consistent_result {
            GetTransactionCountResult::Ok(count) => count,
            GetTransactionCountResult::Err(error) => {
                ic_cdk::trap(&format!("failed to get transaction count for {:?}, error: {:?}",args, error))
            }
        },
        MultiGetTransactionCountResult::Inconsistent(inconsistent_results) => {
            ic_cdk::trap(&format!("inconsistent results when retrieving transaction count for {:?}. Received results: {:?}", args, inconsistent_results))
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct State {
    ethereum_network: EthereumNetwork,
    ecdsa_key_name: EcdsaKeyName,
    ecdsa_public_key: Option<EcdsaPublicKey>,
}

impl State {
    pub fn evm_rpc_services(&self) -> RpcServices {
        match self.ethereum_network {
            EthereumNetwork::Mainnet => RpcServices::EthMainnet(None),
            EthereumNetwork::Sepolia => RpcServices::EthSepolia(None),
        }
    }
}

impl From<InitArg> for State {
    fn from(init_arg: InitArg) -> Self {
        State {
            ethereum_network: init_arg.ethereum_network.unwrap_or_default(),
            ecdsa_key_name: init_arg.ecdsa_key_name.unwrap_or_default(),
            ..Default::default()
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct InitArg {
    pub ethereum_network: Option<EthereumNetwork>,
    pub ecdsa_key_name: Option<EcdsaKeyName>,
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

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
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

pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|s| f(s.borrow().deref()))
}

pub fn mutate_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|s| f(s.borrow_mut().deref_mut()))
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct EcdsaPublicKey {
    public_key: Vec<u8>,
    chain_code: Vec<u8>,
}

impl EcdsaPublicKey {
    pub fn derive(&self, owner: &Principal) -> Self {
        use ic_crypto_extended_bip32::{
            DerivationIndex, DerivationPath, ExtendedBip32DerivationOutput,
        };

        let ExtendedBip32DerivationOutput {
            derived_public_key,
            derived_chain_code,
        } = DerivationPath::new(
            derivation_path(owner)
                .into_iter()
                .map(|x| DerivationIndex(x.into_vec()))
                .collect(),
        )
        .public_key_derivation(&self.public_key, &self.chain_code)
        .expect("BUG: failed to derive an ECDSA public key");
        Self {
            public_key: derived_public_key,
            chain_code: derived_chain_code,
        }
    }
}

fn derivation_path(owner: &Principal) -> Vec<ByteBuf> {
    const SCHEMA_V1: u8 = 1;
    vec![
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(owner.as_slice().to_vec()),
    ]
}

impl From<EcdsaPublicKeyResponse> for EcdsaPublicKey {
    fn from(value: EcdsaPublicKeyResponse) -> Self {
        EcdsaPublicKey {
            public_key: value.public_key,
            chain_code: value.chain_code,
        }
    }
}

impl From<&EcdsaPublicKey> for Address {
    fn from(value: &EcdsaPublicKey) -> Self {
        let public_key = PublicKey::deserialize_sec1(&value.public_key).unwrap_or_else(|e| {
            ic_cdk::trap(&format!("failed to decode minter's public key: {:?}", e))
        });
        ecdsa_public_key_to_address(&public_key)
    }
}

async fn lazy_call_ecdsa_public_key() -> EcdsaPublicKey {
    use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, EcdsaPublicKeyArgument};

    if let Some(ecdsa_pk) = read_state(|s| s.ecdsa_public_key.clone()) {
        return ecdsa_pk;
    }
    let key_name = read_state(|s| s.ecdsa_key_name.clone());
    let (response,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyId::from(key_name),
    })
    .await
    .unwrap_or_else(|(error_code, message)| {
        ic_cdk::trap(&format!(
            "failed to get canister's public key: {} (error code = {:?})",
            message, error_code,
        ))
    });
    let pk = EcdsaPublicKey::from(response);
    mutate_state(|s| s.ecdsa_public_key = Some(pk.clone()));
    pk
}

fn ecdsa_public_key_to_address(pubkey: &PublicKey) -> Address {
    let key_bytes = pubkey.serialize_sec1(/*compressed=*/ false);
    debug_assert_eq!(key_bytes[0], 0x04);
    let hash = keccak(&key_bytes[1..]);
    let mut addr = [0u8; 20];
    addr[..].copy_from_slice(&hash[12..32]);
    Address::new(addr)
}

fn keccak(bytes: &[u8]) -> [u8; 32] {
    ic_crypto_sha3::Keccak256::hash(bytes)
}

fn validate_caller_not_anonymous() -> Principal {
    let principal = ic_cdk::caller();
    if principal == Principal::anonymous() {
        panic!("anonymous principal is not allowed");
    }
    principal
}
