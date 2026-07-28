import EvmRpc "canister:evm_rpc";

module {
  // The EVM RPC canister is imported by name via `canister:evm_rpc`, typed
  // against candid/evm_rpc.did — so the request/response types below are the
  // canister's own (e.g. EvmRpc.Block, EvmRpc.RpcServices), not hand-written.
  // icp-cli injects its principal as PUBLIC_CANISTER_ID:evm_rpc at deploy time;
  // the mops.toml `--actor-env-alias` flag binds the import to that variable.

  // The result type exposed to the main actor — matches the Rust variant names
  // for cross-language consistency, and carries the RPC canister's Block type.
  public type EvmBlockResult = { #Ok : EvmRpc.Block; #Err : Text };

  // Fetches the Ethereum mainnet block at the given height.
  // Uses PublicNode by default — no API key required, works locally and on mainnet.
  // For production deployments requiring premium providers (Alchemy, Ankr, BlockPi),
  // configure API keys via the EVM RPC canister, then pass null to use all configured
  // providers for better consensus: #EthMainnet(null)
  public func getBlock(height : Nat) : async EvmBlockResult {
    let services : EvmRpc.RpcServices = #EthMainnet(?[#PublicNode]);

    // To query a different chain, use #Custom instead:
    // let services : EvmRpc.RpcServices = #Custom {
    //   chainId = 8453; // Base Mainnet — see https://chainlist.org/ for chain IDs
    //   services = [{ url = "https://base-rpc.publicnode.com"; headers = null }];
    // };

    let result = await (with cycles = 10_000_000_000) EvmRpc.eth_getBlockByNumber(services, null, #Number height);

    switch result {
      case (#Consistent(#Ok block)) { #Ok block };
      case (#Consistent(#Err err)) { #Err(debug_show err) };
      case (#Inconsistent(v)) { #Err("RPC providers gave inconsistent results: " # debug_show v) };
    };
  };
};
