import Debug "mo:base/Debug";
import { abs } = "mo:base/Int";
import { now } = "mo:base/Time";
import { setTimer; recurringTimer } = "mo:base/Timer";
import StableMemory "mo:base/ExperimentalStableMemory";

actor CanisterLogs {

  let timerDelaySeconds = 5;
  let second = 1_000_000_000;
  let ic00_raw_rand = (actor "aaaaa-aa" : actor { raw_rand : () -> async Blob }).raw_rand;

  private func execute_timer() : async () {
    Debug.print("right before timer trap");
    Debug.trap("timer trap");
  };

  ignore setTimer<system>(#seconds (timerDelaySeconds - abs(now() / second) % timerDelaySeconds),
    func () : async () {
      ignore recurringTimer<system>(#seconds timerDelaySeconds, execute_timer);
      await execute_timer();
  });

  public func print(text : Text) : async () {
    Debug.print(text);
  };

  public query func print_query(text : Text) : async () {
    Debug.print(text);
  };

  public func trap(text : Text) : async () {
    Debug.print("right before trap");
    Debug.trap(text);
  };

  public query func trap_query(text : Text) : async () {
    Debug.print("right before trap_query");
    Debug.trap(text);
  };

  public func memory_oob() : async () {
    Debug.print("right before memory out of bounds");
    let offset : Nat64 = 10;
    let value : Nat = 20;
    let _blob = StableMemory.loadBlob(offset, value);  // Expect reading outside of memory bounds to trap.
  };

  public func raw_rand() : async Blob {
    Debug.print("pre ic.raw_rand() call");
    let bytes = await ic00_raw_rand();
    Debug.print("ic.raw_rand() call succeeded");
    bytes;
  };

};
