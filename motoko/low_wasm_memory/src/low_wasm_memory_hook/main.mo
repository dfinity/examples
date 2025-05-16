import Array "mo:base/Array";
import Deque "mo:base/Deque";
import Buffer "mo:base/Buffer";

actor {
  // Types
  type FnType = {
    #heartbeat;
    #onLowWasmMemory;
  };

  // State
  var fnOrderBuffer = Buffer.Buffer<FnType>(30);
  var bytes : Deque.Deque<[Nat]> = Deque.empty();
  var hookExecuted : Bool = false;

  // Query function to get execution order
  public query func getExecutedFunctionsOrder() : async [FnType] {
    Buffer.toArray(fnOrderBuffer);
  };

  // Heartbeat function that increases memory usage
  system func heartbeat() : async () {
    if (not hookExecuted) {
      fnOrderBuffer.add(#heartbeat);
      // Allocate more memory by creating a large array
      let chunk = Array.tabulate<Nat>(10_000, func _ = 0);
      bytes := Deque.pushBack(bytes, chunk);
    };
  };

  // Low WASM memory hook
  system func lowmemory() : async* () {
    hookExecuted := true;
    fnOrderBuffer.add(#onLowWasmMemory);
  };
};
