import Nat "mo:core/Nat";
import Nat64 "mo:core/Nat64";
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
  /// Returns how many cycles were actually accepted by this canister.
  ///
  /// This is the RECEIVER perspective: the canister decides how much to keep.
  public func acceptCycles() : async { accepted : Nat64 } {
    let available = Cycles.available(); // total cycles the caller attached
    let accepted = Cycles.accept<system>(Nat.min(available, limit)); // claim up to limit
    { accepted = Nat64.fromNat(accepted) };
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
