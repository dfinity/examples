import Debug "mo:core/Debug";
import Int "mo:core/Int";
import Region "mo:core/Region";
import Runtime "mo:core/Runtime";
import { now } = "mo:core/Time";
import { setTimer; recurringTimer } = "mo:core/Timer";

persistent actor CanisterLogs {

  transient let timerDelaySeconds = 5;
  transient let second = 1_000_000_000;
  transient let ic00_raw_rand = (actor "aaaaa-aa" : actor { raw_rand : () -> async Blob }).raw_rand;

  private func execute_timer() : async () {
    "right before timer trap".print();
    "timer trap".trap();
  };

  ignore setTimer<system>(
    #seconds(timerDelaySeconds - (now() / second).abs() % timerDelaySeconds),
    func() : async () {
      ignore recurringTimer<system>(#seconds timerDelaySeconds, execute_timer);
      await execute_timer();
    },
  );

  public func print(text : Text) : async () {
    text.print();
  };

  public query func print_query(text : Text) : async () {
    text.print();
  };

  public func trap(text : Text) : async () {
    "right before trap".print();
    text.trap();
  };

  public query func trap_query(text : Text) : async () {
    "right before trap_query".print();
    text.trap();
  };

  public func memory_oob() : async () {
    "right before memory out of bounds".print();
    let region = Region.new();
    let offset : Nat64 = 10;
    let size : Nat = 20;
    let _blob = region.loadBlob(offset, size); // Expect reading outside of memory bounds to trap.
  };

  public func raw_rand() : async Blob {
    "pre ic.raw_rand() call".print();
    let bytes = await ic00_raw_rand();
    "ic.raw_rand() call succeeded".print();
    bytes;
  };

};
