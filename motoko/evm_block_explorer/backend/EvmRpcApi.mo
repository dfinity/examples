import Runtime "mo:core/Runtime";

module {
  // Inline actor type for the EVM RPC canister, exposing only eth_getBlockByNumber.
  // Full Candid interface: https://github.com/dfinity/evm-rpc-canister/releases/latest/download/evm_rpc.did

  public type Block = {
    miner : Text;
    totalDifficulty : ?Nat;
    receiptsRoot : Text;
    stateRoot : Text;
    hash : Text;
    difficulty : ?Nat;
    size : Nat;
    uncles : [Text];
    baseFeePerGas : ?Nat;
    extraData : Text;
    transactionsRoot : ?Text;
    sha3Uncles : Text;
    nonce : Nat;
    number : Nat;
    timestamp : Nat;
    transactions : [Text];
    gasLimit : Nat;
    logsBloom : Text;
    parentHash : Text;
    gasUsed : Nat;
    mixHash : Text;
  };

  type BlockTag = {
    #Earliest;
    #Safe;
    #Finalized;
    #Latest;
    #Number : Nat;
    #Pending;
  };

  type HttpHeader = { value : Text; name : Text };
  type RpcApi = { url : Text; headers : ?[HttpHeader] };

  type EthMainnetService = {
    #Alchemy;
    #Ankr;
    #BlockPi;
    #Cloudflare;
    #PublicNode;
    #Llama;
  };

  type EthSepoliaService = {
    #Alchemy;
    #Ankr;
    #BlockPi;
    #PublicNode;
    #Sepolia;
  };

  type L2MainnetService = {
    #Alchemy;
    #Ankr;
    #BlockPi;
    #PublicNode;
    #Llama;
  };

  type RpcServices = {
    #Custom : { chainId : Nat64; services : [RpcApi] };
    #EthSepolia : ?[EthSepoliaService];
    #EthMainnet : ?[EthMainnetService];
    #ArbitrumOne : ?[L2MainnetService];
    #BaseMainnet : ?[L2MainnetService];
    #OptimismMainnet : ?[L2MainnetService];
  };

  type RpcService = {
    #Provider : Nat64;
    #Custom : RpcApi;
    #EthSepolia : EthSepoliaService;
    #EthMainnet : EthMainnetService;
    #ArbitrumOne : L2MainnetService;
    #BaseMainnet : L2MainnetService;
    #OptimismMainnet : L2MainnetService;
  };

  type RejectionCode = {
    #NoError;
    #CanisterError;
    #SysTransient;
    #DestinationInvalid;
    #Unknown;
    #SysFatal;
    #CanisterReject;
  };

  type JsonRpcError = { code : Int64; message : Text };

  type ProviderError = {
    #TooFewCycles : { expected : Nat; received : Nat };
    #MissingRequiredProvider;
    #ProviderNotFound;
    #NoPermission;
    #InvalidRpcConfig : Text;
  };

  type HttpOutcallError = {
    #IcError : { code : RejectionCode; message : Text };
    #InvalidHttpJsonRpcResponse : { status : Nat16; body : Text; parsingError : ?Text };
  };

  type ValidationError = {
    #Custom : Text;
    #InvalidHex : Text;
  };

  type RpcError = {
    #JsonRpcError : JsonRpcError;
    #ProviderError : ProviderError;
    #ValidationError : ValidationError;
    #HttpOutcallError : HttpOutcallError;
  };

  type GetBlockByNumberResult = { #Ok : Block; #Err : RpcError };

  type MultiGetBlockByNumberResult = {
    #Consistent : GetBlockByNumberResult;
    #Inconsistent : [(RpcService, GetBlockByNumberResult)];
  };

  type EvmRpcActor = actor {
    // Pass null for RpcConfig — unused cycles are refunded by the EVM RPC canister.
    eth_getBlockByNumber : (RpcServices, ?{}, BlockTag) -> async MultiGetBlockByNumberResult;
  };

  // The result type exposed to the main actor — matches the Rust variant names for cross-language consistency.
  public type EvmBlockResult = { #Ok : Block; #Err : Text };

  // Returns the EVM RPC canister actor, resolved at runtime from the PUBLIC_CANISTER_ID:evm_rpc
  // environment variable. icp-cli sets this automatically at deploy time:
  //   - locally: the principal of the locally deployed evm_rpc canister
  //   - on ICP mainnet (ic environment): 7hfb6-caaaa-aaaar-qadga-cai
  func evmRpc<system>() : EvmRpcActor {
    let ?id = Runtime.envVar<system>("PUBLIC_CANISTER_ID:evm_rpc") else
      Runtime.trap("PUBLIC_CANISTER_ID:evm_rpc not set — run icp deploy");
    actor(id) : EvmRpcActor;
  };

  // Fetches the Ethereum mainnet block at the given height.
  // Uses PublicNode by default — no API key required, works locally and on mainnet.
  // For production deployments requiring premium providers (Alchemy, Ankr, BlockPi),
  // configure API keys via the EVM RPC canister, then pass null to use all configured
  // providers for better consensus: #EthMainnet(null)
  public func getBlock<system>(height : Nat) : async EvmBlockResult {
    let services : RpcServices = #EthMainnet(?[#PublicNode]);

    // To query a different chain, use #Custom instead:
    // let services : RpcServices = #Custom {
    //   chainId = 8453; // Base Mainnet — see https://chainlist.org/ for chain IDs
    //   services = [{ url = "https://base-rpc.publicnode.com"; headers = null }];
    // };

    let result = await (with cycles = 10_000_000_000) evmRpc<system>().eth_getBlockByNumber(services, null, #Number height);

    switch result {
      case (#Consistent(#Ok block)) { #Ok block };
      case (#Consistent(#Err err)) { #Err(debug_show err) };
      case (#Inconsistent(v)) { #Err("RPC providers gave inconsistent results: " # debug_show v) };
    };
  };
};
