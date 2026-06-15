import Nat "mo:core/Nat";
import Cycles "mo:core/Cycles";

actor HelloCycles {

  /// Maximum cycles this canister accepts per call.
  let limit = 10_000_000;

  /// Returns the canister's current cycle balance.
  public query func getBalance() : async Nat {
    Cycles.balance();
  };

  /// Accepts cycles that the caller attached to this call, up to `limit`.
  /// The remainder is automatically refunded to the caller.
  ///
  /// Returns `()` so that it can be passed as a `shared () -> async ()` receiver
  /// to `sendCycles`.  Use `getBalance` before and after to observe how many
  /// cycles were accepted.
  ///
  /// This is the RECEIVER perspective: the canister decides how much to keep.
  public func acceptCycles() : async () {
    let available = Cycles.available(); // total cycles the caller attached
    ignore Cycles.accept<system>(Nat.min(available, limit)); // claim up to limit
  };

  /// Sends `amount` cycles from this canister's balance to `receiver`.
  /// Returns how many cycles were refunded (not accepted by the receiver).
  ///
  /// This is the SENDER perspective: the canister spends from its own
  /// balance and learns how many cycles came back unused.
  public func sendCycles(receiver : shared () -> async (), amount : Nat) : async { refunded : Nat } {
    await (with cycles = amount) receiver(); // attach `amount` to the outgoing call
    { refunded = Cycles.refunded() };        // how many the receiver did not accept
  };

};
