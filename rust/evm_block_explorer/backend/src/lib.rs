use candid::Principal;
use evm_rpc_types::{Block, BlockTag, EthMainnetService, MultiRpcResult, Nat256, RpcServices};
use ic_cdk_management_canister::{
    ecdsa_public_key, schnorr_public_key, sign_with_ecdsa, sign_with_schnorr, EcdsaCurve,
    EcdsaKeyId, EcdsaPublicKeyArgs, SchnorrAlgorithm, SchnorrKeyId, SchnorrPublicKeyArgs,
    SignWithEcdsaArgs, SignWithSchnorrArgs,
};

const KEY_NAME: &str = "test_key_1";

/// Resolve the EVM RPC canister ID at runtime. icp-cli sets the
/// `PUBLIC_CANISTER_ID:evm_rpc` env var for local deployments; on production
/// it is explicitly configured in icp.yaml to `7hfb6-caaaa-aaaar-qadga-cai`.
fn evm_rpc_id() -> Principal {
    let id = ic_cdk::api::env_var_value("PUBLIC_CANISTER_ID:evm_rpc");
    Principal::from_text(&id).expect("Invalid PUBLIC_CANISTER_ID:evm_rpc")
}

#[ic_cdk::update]
async fn get_evm_block(height: u128) -> Result<Block, String> {
    // Uses PublicNode by default — no API key required, works locally and on mainnet.
    // For production deployments requiring premium providers (Alchemy, Ankr, BlockPi),
    // configure API keys via the EVM RPC canister, then pass None to use all configured
    // providers for better consensus:
    //   RpcServices::EthMainnet(None)
    let rpc_services = RpcServices::EthMainnet(Some(vec![EthMainnetService::PublicNode]));

    // To query a different chain, use RpcServices::Custom instead:
    // let rpc_services = RpcServices::Custom {
    //     chain_id: 8453, // Base Mainnet — see https://chainlist.org/ for chain IDs
    //     services: vec![evm_rpc_types::RpcApi {
    //         url: "https://base-rpc.publicnode.com".to_string(),
    //         headers: None,
    //     }],
    // };

    let client = evm_rpc_client::EvmRpcClient::builder(
        ic_canister_runtime::IcRuntime::new(),
        evm_rpc_id(),
    )
    .with_rpc_sources(rpc_services)
    .build();

    // Call `eth_getBlockByNumber` RPC method (unused cycles will be refunded)
    let result: MultiRpcResult<Block> = client
        .get_block_by_number(BlockTag::Number(Nat256::from(height)))
        .send()
        .await;

    match result {
        MultiRpcResult::Consistent(Ok(block)) => Ok(block),
        MultiRpcResult::Consistent(Err(err)) => Err(format!("{err:?}")),
        MultiRpcResult::Inconsistent(v) => Err(format!("RPC providers gave inconsistent results: {v:?}")),
    }
}

fn sha256(input: &String) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}

#[ic_cdk::update]
async fn get_ecdsa_public_key() -> String {
    let pub_key = ecdsa_public_key(&EcdsaPublicKeyArgs {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
    })
    .await
    .expect("Failed to get ecdsa public key");
    hex::encode(pub_key.public_key)
}

#[ic_cdk::update]
async fn get_schnorr_public_key() -> String {
    let pub_key = schnorr_public_key(&SchnorrPublicKeyArgs {
        canister_id: None,
        derivation_path: vec![],
        key_id: SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name: KEY_NAME.to_string(),
        },
    })
    .await
    .expect("Failed to get schnorr public key");
    hex::encode(pub_key.public_key)
}

#[ic_cdk::update]
async fn sign_message_with_ecdsa(message: String) -> String {
    let message_hash = sha256(&message);
    let signature = sign_with_ecdsa(&SignWithEcdsaArgs {
        message_hash: message_hash.to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
    })
    .await
    .expect("Failed to sign with ecdsa");
    hex::encode(signature.signature)
}

#[ic_cdk::update]
async fn sign_message_with_schnorr(message: String) -> String {
    let message = sha256(&message);
    let signature = sign_with_schnorr(&SignWithSchnorrArgs {
        message: message.to_vec(),
        derivation_path: vec![],
        key_id: SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name: KEY_NAME.to_string(),
        },
        aux: None,
    })
    .await
    .expect("Failed to sign with schnorr");
    hex::encode(signature.signature)
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
