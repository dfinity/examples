mod ecdsa;
mod ethereum_wallet;
mod state;

use crate::ethereum_wallet::EthereumWallet;
use crate::state::{init_state, read_state};
use alloy_consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy_primitives::{hex, Signature, TxKind, U256};
use candid::{CandidType, Deserialize, Nat, Principal};
use evm_rpc_types::{
    BlockTag, EthMainnetService, EthSepoliaService, GetTransactionCountArgs, Hex20,
    MultiRpcResult, Nat256, RpcService,
};
use ic_cdk_management_canister::{EcdsaCurve, EcdsaKeyId};
use ic_cdk::{init, update};
use ic_ethereum_types::Address;
use num::{BigUint, Num};
use std::str::FromStr;

// The EVM RPC canister ID is configured as a canister environment variable:
//   local:      PUBLIC_CANISTER_ID:evm_rpc injected by icp-cli after deploying the pre-built canister
//   production: EVM_RPC_CANISTER_ID = 7hfb6-caaaa-aaaar-qadga-cai (shared mainnet EVM RPC)
//
// See icp.yaml for the environment configuration.
fn evm_rpc_id() -> Principal {
    let id = ic_cdk::api::env_var_value("PUBLIC_CANISTER_ID:evm_rpc");
    Principal::from_text(&id).expect("invalid EVM_RPC_CANISTER_ID")
}

#[init]
pub fn init(maybe_init: Option<InitArg>) {
    if let Some(init_arg) = maybe_init {
        init_state(init_arg)
    }
}

#[update]
pub async fn ethereum_address(owner: Option<Principal>) -> String {
    let caller = validate_caller_not_anonymous();
    let owner = owner.unwrap_or(caller);
    let wallet = EthereumWallet::new(owner).await;
    wallet.ethereum_address().to_string()
}

#[update]
pub async fn get_balance(address: Option<String>) -> Nat {
    let address = address.unwrap_or(ethereum_address(None).await);

    let json = format!(
        r#"{{ "jsonrpc": "2.0", "method": "eth_getBalance", "params": ["{}", "latest"], "id": 1 }}"#,
        address
    );

    let max_response_size_bytes = 500_u64;
    let num_cycles = 1_000_000_000u128;

    let ethereum_network = read_state(|s| s.ethereum_network());

    let rpc_service = match ethereum_network {
        EthereumNetwork::Mainnet => RpcService::EthMainnet(EthMainnetService::PublicNode),
        EthereumNetwork::Sepolia => RpcService::EthSepolia(EthSepoliaService::PublicNode),
    };

    use evm_rpc_types::RpcResult;
    let (response,): (RpcResult<String>,) = ic_cdk::call::Call::bounded_wait(evm_rpc_id(), "request")
        .with_args(&(rpc_service, json, max_response_size_bytes))
        .with_cycles(num_cycles)
        .await
        .expect("RPC call failed")
        .candid_tuple()
        .expect("failed to decode response");

    let hex_balance = match response {
        Ok(balance_result) => {
            // The response to a successful `eth_getBalance` call has the following format:
            // { "id": "[ID]", "jsonrpc": "2.0", "result": "[BALANCE IN HEX]" }
            let response: serde_json::Value = serde_json::from_str(&balance_result).unwrap();
            response
                .get("result")
                .and_then(|v| v.as_str())
                .unwrap()
                .to_string()
        }
        Err(e) => panic!("Received an error response: {:?}", e),
    };

    // Remove the "0x" prefix before converting to a decimal number.
    Nat(BigUint::from_str_radix(&hex_balance[2..], 16).unwrap())
}

#[update]
pub async fn transaction_count(owner: Option<Principal>, block: Option<BlockTag>) -> Nat {
    let caller = validate_caller_not_anonymous();
    let owner = owner.unwrap_or(caller);
    let wallet = EthereumWallet::new(owner).await;
    let rpc_services = read_state(|s| s.evm_rpc_services());
    let address: Hex20 = wallet
        .ethereum_address()
        .to_string()
        .parse()
        .expect("failed to parse ethereum address");
    let args = GetTransactionCountArgs {
        address,
        block: block.unwrap_or(BlockTag::Finalized),
    };
    let (result,): (MultiRpcResult<Nat256>,) =
        ic_cdk::call::Call::bounded_wait(evm_rpc_id(), "eth_getTransactionCount")
            .with_args(&(rpc_services, Option::<evm_rpc_types::RpcConfig>::None, args.clone()))
            .with_cycles(2_000_000_000_u128)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "failed to get transaction count for {:?}, error: {:?}",
                    args, e
                )
            })
            .candid_tuple()
            .expect("failed to decode response");

    match result {
        MultiRpcResult::Consistent(consistent_result) => match consistent_result {
            Ok(count) => Nat(count.as_ref().0.clone()),
            Err(error) => {
                ic_cdk::trap(&format!("failed to get transaction count for {:?}, error: {:?}", args, error))
            }
        },
        MultiRpcResult::Inconsistent(inconsistent_results) => {
            ic_cdk::trap(&format!("inconsistent results when retrieving transaction count for {:?}. Received results: {:?}", args, inconsistent_results))
        }
    }
}

/// Demonstrates the high-level `EvmRpcClient` pattern: no manual cycle amounts, automatic
/// consensus across providers, and a cleaner API surface compared to the raw inter-canister
/// call used in `transaction_count`. Accepts any Ethereum address directly (like `get_balance`),
/// rather than deriving an address from an IC principal.
#[update]
pub async fn transaction_count_with_client(address: Option<String>, block: Option<BlockTag>) -> Nat {
    let address = address.unwrap_or(ethereum_address(None).await);
    let rpc_services = read_state(|s| s.evm_rpc_services());

    let address: Hex20 = address.parse().expect("failed to parse ethereum address");
    let block_tag = block.unwrap_or(BlockTag::Finalized);

    let canister_id = evm_rpc_id();
    let client = evm_rpc_client::EvmRpcClient::builder(
        ic_canister_runtime::IcRuntime::new(),
        canister_id,
    )
    .with_rpc_sources(rpc_services)
    .build();

    let result: MultiRpcResult<Nat256> = client
        .get_transaction_count((address, block_tag))
        .send()
        .await;

    match result {
        MultiRpcResult::Consistent(Ok(count)) => Nat(count.as_ref().0.clone()),
        MultiRpcResult::Consistent(Err(error)) => {
            ic_cdk::trap(&format!("failed to get transaction count, error: {:?}", error))
        }
        MultiRpcResult::Inconsistent(inconsistent_results) => {
            ic_cdk::trap(&format!(
                "inconsistent results when retrieving transaction count. Received results: {:?}",
                inconsistent_results
            ))
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
    let chain_id = read_state(|s| s.ethereum_network().chain_id());
    let nonce = nat_to_u64(transaction_count(Some(caller), Some(BlockTag::Latest)).await);
    let (gas_limit, max_fee_per_gas, max_priority_fee_per_gas) = estimate_transaction_fees();

    let transaction = TxEip1559 {
        chain_id,
        nonce,
        gas_limit,
        max_fee_per_gas,
        max_priority_fee_per_gas,
        to: TxKind::Call(to.parse().expect("failed to parse recipient address")),
        value: nat_to_u256(amount),
        access_list: Default::default(),
        input: Default::default(),
    };

    let wallet = EthereumWallet::new(caller).await;
    let tx_hash = transaction.signature_hash().0;
    let (raw_signature, recovery_id) = wallet.sign_with_ecdsa(tx_hash).await;
    let signature = Signature::from_bytes_and_parity(&raw_signature, recovery_id.is_y_odd())
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
    // but in production multiple providers (e.g., using a round-robin strategy) should be used to avoid a single point of failure.
    let single_rpc_service = read_state(|s| s.single_evm_rpc_service());

    use evm_rpc_types::{MultiRpcResult as SendResult, SendRawTransactionStatus};
    let (result,): (SendResult<SendRawTransactionStatus>,) =
        ic_cdk::call::Call::bounded_wait(evm_rpc_id(), "eth_sendRawTransaction")
            .with_args(&(single_rpc_service, Option::<evm_rpc_types::RpcConfig>::None, raw_transaction_hex.clone()))
            .with_cycles(2_000_000_000_u128)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "failed to send raw transaction {}, error: {:?}",
                    raw_transaction_hex, e
                )
            })
            .candid_tuple()
            .expect("failed to decode response");

    ic_cdk::println!(
        "Result of sending raw transaction {}: {:?}. \
    Due to the replicated nature of HTTPs outcalls, an error such as transaction already known or nonce too low could be reported, \
    even though the transaction was successfully sent. \
    Check whether the transaction appears on Etherscan or check that the transaction count on \
    that address at latest block height did increase.",
        raw_transaction_hex,
        result
    );

    raw_transaction_hash.to_string()
}

fn estimate_transaction_fees() -> (u128, u128, u128) {
    /// Standard gas limit for an Ethereum transfer to an EOA.
    /// Other transactions, in particular ones interacting with a smart contract (e.g., ERC-20), would require a higher gas limit.
    const GAS_LIMIT: u128 = 21_000;

    /// Very crude estimates of max_fee_per_gas and max_priority_fee_per_gas.
    /// A real world application would need to estimate this more accurately by for example fetching the fee history from the last 5 blocks.
    const MAX_FEE_PER_GAS: u128 = 50_000_000_000;
    const MAX_PRIORITY_FEE_PER_GAS: u128 = 1_500_000_000;
    (GAS_LIMIT, MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS)
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct InitArg {
    pub ethereum_network: Option<EthereumNetwork>,
    pub ecdsa_key_name: Option<EcdsaKeyName>,
}

#[derive(CandidType, Deserialize, Debug, Default, PartialEq, Eq, Clone, Copy)]
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

impl From<&EcdsaKeyName> for EcdsaKeyId {
    fn from(value: &EcdsaKeyName) -> Self {
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

pub fn validate_caller_not_anonymous() -> Principal {
    let principal = ic_cdk::api::msg_caller();
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

ic_cdk::export_candid!();
