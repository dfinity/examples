import Debug "mo:core/Debug";
import { abs } = "mo:core/Int";
import Region "mo:core/Region";
import Runtime "mo:core/Runtime";
import { now } = "mo:core/Time";
import { setTimer; recurringTimer } = "mo:core/Timer";

persistent actor CanisterLogs {

  transient let timerDelaySeconds = 5;
  transient let second = 1_000_000_000;
  transient let ic00_raw_rand = (actor "aaaaa-aa" : actor { raw_rand : () -> async Blob }).raw_rand;

  private func execute_timer() : async () {
    Debug.print("right before timer trap");
    Runtime.trap("timer trap");
  };

  ignore setTimer<system>(
    #seconds(timerDelaySeconds - abs(now() / second) % timerDelaySeconds),
    func() : async () {
      ignore recurringTimer<system>(#seconds timerDelaySeconds, execute_timer);
      await execute_timer();
    },
  );

  public func print(text : Text) : async () {
    Debug.print(text);
  };

  public query func print_query(text : Text) : async () {
    Debug.print(text);
  };

  public func trap(text : Text) : async () {
    Debug.print("right before trap");
    Runtime.trap(text);
  };

  public query func trap_query(text : Text) : async () {
    Debug.print("right before trap_query");
    Runtime.trap(text);
  };

  public func memory_oob() : async () {
    Debug.print("right before memory out of bounds");
    let region = Region.new();
    let offset : Nat64 = 10;
    let size : Nat = 20;
    let _blob = region.loadBlob(offset, size); // Expect reading outside of memory bounds to trap.
  };

  public func raw_rand() : async Blob {
    Debug.print("pre ic.raw_rand() call");
    let bytes = await ic00_raw_rand();
    Debug.print("ic.raw_rand() call succeeded");
    bytes;
  };

};
