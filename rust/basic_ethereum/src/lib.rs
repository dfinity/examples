use alloy_consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy_primitives::{hex, Signature, TxKind, U256};
use candid::{CandidType, Deserialize, Nat, Principal};
use evm_rpc_canister_types::{
    BlockTag, EthMainnetService, EthSepoliaService, EvmRpcCanister, GetTransactionCountArgs,
    GetTransactionCountResult, MultiGetTransactionCountResult, RpcServices,
};
use ic_cdk::api::management_canister::ecdsa::{
    EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyResponse, SignWithEcdsaArgument,
};
use ic_cdk::{init, update};
use ic_crypto_ecdsa_secp256k1::{PublicKey, RecoveryId};
use ic_ethereum_types::Address;
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

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

#[update]
pub async fn send_eth(to: String, amount: Nat) -> String {
    use alloy_eips::eip2718::Encodable2718;

    let caller = validate_caller_not_anonymous();
    let _to_address = Address::from_str(&to).unwrap_or_else(|e| {
        ic_cdk::trap(&format!("failed to parse the recipient address: {:?}", e))
    });
    let chain_id = read_state(|s| s.ethereum_network.chain_id());
    let nonce = nat_to_u64(transaction_count(Some(caller), Some(BlockTag::Latest)).await);

    let transaction = TxEip1559 {
        chain_id,
        nonce,
        gas_limit: 21_000,
        max_fee_per_gas: 50_000_000_000,
        max_priority_fee_per_gas: 1_500_000_000,
        to: TxKind::Call(to.parse().expect("failed to parse recipient address")),
        value: nat_to_u256(amount),
        access_list: Default::default(),
        input: Default::default(),
    };

    let tx_hash = transaction.signature_hash().0;
    let derivation_path = derivation_path(&caller)
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let raw_sig = sign_with_ecdsa(
        read_state(|s| s.ecdsa_key_name.clone()),
        derivation_path,
        tx_hash,
    )
    .await;
    let recid = compute_recovery_id(&tx_hash, &caller, &raw_sig).await;
    if recid.is_x_reduced() {
        ic_cdk::trap("BUG: affine x-coordinate of r is reduced which is so unlikely to happen that it's probably a bug");
    }
    let signature = Signature::from_bytes_and_parity(&raw_sig, recid.is_y_odd())
        .expect("BUG: failed to create a signature");
    let signed_tx = transaction.into_signed(signature);

    let raw_transaction_hash = *signed_tx.hash();
    let mut tx_bytes: Vec<u8> = vec![];
    TxEnvelope::from(signed_tx).encode_2718(&mut tx_bytes);
    let raw_transaction_hex = format!("0x{}", hex::encode(&tx_bytes));
    ic_cdk::println!(
        "Sending raw transaction hex {} with transaction hash {}",
        raw_transaction_hex,
        raw_transaction_hash
    );
    // The canister is sending a signed statement, meaning a malicious provider could only affect availability.
    // For demonstration purposes, the canister uses a single provider to send the signed transaction,
    // but in production multiple providers (e.g., using round-robin strategy) should be used to avoid a single point of failure.
    let single_rpc_service = read_state(|s| s.single_evm_rpc_service());
    let (result,) = EVM_RPC
        .eth_send_raw_transaction(
            single_rpc_service,
            None,
            raw_transaction_hex.clone(),
            2_000_000_000_u128,
        )
        .await
        .unwrap_or_else(|e| {
            panic!(
                "failed to send raw transaction {}, error: {:?}",
                raw_transaction_hex, e
            )
        });
    ic_cdk::println!(
        "Result of sending raw transaction {}: {:?}. \
    Due to the replicated nature of HTTPs outcalls an error there is the most likely outcome. \
    Check whether the transaction appears on Etherscan or check that the transaction count on \
    that address as latest block height did increase.",
        raw_transaction_hex,
        result
    );

    raw_transaction_hash.to_string()
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

    pub fn single_evm_rpc_service(&self) -> RpcServices {
        match self.ethereum_network {
            EthereumNetwork::Mainnet => {
                RpcServices::EthMainnet(Some(vec![EthMainnetService::Ankr]))
            }
            EthereumNetwork::Sepolia => {
                RpcServices::EthSepolia(Some(vec![EthSepoliaService::Ankr]))
            }
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

/// Signs a message hash using the tECDSA API.
pub async fn sign_with_ecdsa(
    key_name: EcdsaKeyName,
    derivation_path: Vec<Vec<u8>>,
    message_hash: [u8; 32],
) -> [u8; 64] {
    let (result,) =
        ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument {
            message_hash: message_hash.to_vec(),
            derivation_path,
            key_id: EcdsaKeyId::from(key_name),
        })
        .await
        .expect("failed to sign with ecdsa");

    let signature_length = result.signature.len();
    <[u8; 64]>::try_from(result.signature).unwrap_or_else(|_| {
        panic!(
            "BUG: invalid signature from management canister. Expected 64 bytes but got {} bytes",
            signature_length
        )
    })
}

async fn compute_recovery_id(digest: &[u8; 32], owner: &Principal, signature: &[u8]) -> RecoveryId {
    let ecdsa_public_key = lazy_call_ecdsa_public_key().await.derive(owner);
    let ecdsa_public_key = PublicKey::deserialize_sec1(&ecdsa_public_key.public_key)
        .unwrap_or_else(|e| {
            ic_cdk::trap(&format!("failed to decode minter's public key: {:?}", e))
        });

    assert!(
        ecdsa_public_key.verify_signature_prehashed(digest, signature),
        "failed to verify signature prehashed, digest: {:?}, signature: {:?}, public_key: {:?}",
        hex::encode(digest),
        hex::encode(signature),
        hex::encode(ecdsa_public_key.serialize_sec1(true)),
    );
    ecdsa_public_key
        .try_recovery_from_digest(digest, signature)
        .unwrap_or_else(|e| {
            panic!(
                "BUG: failed to recover public key {:?} from digest {:?} and signature {:?}: {:?}",
                hex::encode(ecdsa_public_key.serialize_sec1(true)),
                hex::encode(digest),
                hex::encode(signature),
                e
            )
        })
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

fn nat_to_u64(nat: Nat) -> u64 {
    use num_traits::cast::ToPrimitive;
    nat.0
        .to_u64()
        .unwrap_or_else(|| ic_cdk::trap(&format!("Nat {} doesn't fit into a u64", nat)))
}

fn nat_to_u256(value: Nat) -> U256 {
    let value_bytes = value.0.to_bytes_be();
    assert!(
        value_bytes.len() <= 32,
        "Nat does not fit in a U256: {}",
        value
    );
    let mut value_u256 = [0u8; 32];
    value_u256[32 - value_bytes.len()..].copy_from_slice(&value_bytes);
    U256::from_be_bytes(value_u256)
}
