import Debug "mo:base/Debug";
import { abs } = "mo:base/Int";
import { now } = "mo:base/Time";
import { setTimer; recurringTimer } = "mo:base/Timer";

actor CanisterLogs {

  let timerDelaySeconds = 5;

  private func execute_timer() : async () {
    Debug.print("right before timer trap");
    Debug.trap("timer trap");
  };

  ignore setTimer<system>(#seconds (timerDelaySeconds - abs(now() / 1_000_000_000) % timerDelaySeconds),
    func () : async () {
      ignore recurringTimer<system>(#seconds timerDelaySeconds, execute_timer);
      await execute_timer();
  });

  public func print(text : Text) : async () {
    Debug.print(text);
  };

  public func trap(text : Text) : async () {
    Debug.print("right before trap");
    Debug.trap(text);
  };

};
