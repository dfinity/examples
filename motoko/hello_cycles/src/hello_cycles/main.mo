import Nat "mo:core/Nat";
import Nat64 "mo:core/Nat64";
import Cycles "mo:core/Cycles";

actor HelloCycles  {

  let limit = 10_000_000;

  public func wallet_balance() : async Nat {
    return Cycles.balance();
  };

  public func wallet_receive() : async { accepted: Nat64 } {
    let available = Cycles.available();
    let accepted = Cycles.accept(Nat.min(available, limit));
    { accepted = Nat64.fromNat(accepted) };
  };

  public func transfer(
    receiver : shared () -> async (),
    amount : Nat) : async { refunded : Nat } {
      Cycles.add(amount);
      await receiver();
      { refunded = Cycles.refunded() };
  };

};
