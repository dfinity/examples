import Array "mo:core/Array";
import Queue "mo:core/Queue";
import List "mo:core/List";

persistent actor {
  // Types
  type FnType = {
    #heartbeat;
    #onLowWasmMemory;
  };

  // State
  transient let fnOrderBuffer = List.empty<FnType>();
  transient let bytes = Queue.empty<[Nat]>();
  transient var hookExecuted : Bool = false;

  // Query function to get execution order
  public query func getExecutedFunctionsOrder() : async [FnType] {
    fnOrderBuffer.toArray()
  };

  // Heartbeat function that increases memory usage
  system func heartbeat() : async () {
    if (not hookExecuted) {
      fnOrderBuffer.add(#heartbeat);
      // Allocate more memory by creating a large array
      let chunk = Array.tabulate(10_000, func _ = 0);
      bytes.pushBack(chunk);
    };
  };

  // Low WASM memory hook
  system func lowmemory() : async* () {
    hookExecuted := true;
    fnOrderBuffer.add(#onLowWasmMemory);
  };
};
