use candid::Principal;
use evm_rpc_types::{Block, BlockTag, EthMainnetService, MultiRpcResult, Nat256, RpcServices};

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

ic_cdk::export_candid!();
