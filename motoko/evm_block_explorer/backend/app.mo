import EvmRpcApi "EvmRpcApi";

persistent actor EvmBlockExplorer {

  public func get_evm_block(height : Nat) : async EvmRpcApi.EvmBlockResult {
    await EvmRpcApi.getBlock<system>(height);
  };

};
