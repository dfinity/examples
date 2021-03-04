import Nat64 "mo:base/Nat64";
import Cycles "mo:base/ExperimentalCycles";

shared(msg) actor class HelloFunds (
  capacity: Nat
  ) {

  let owner = msg.caller;

  var balance = 0;

  public shared(msg) func wallet_balance() : async Nat {
    return balance;
  };

  public func wallet_receive() : async { accepted: Nat64 } {
    let amount = Cycles.available();
    let limit = capacity - balance;
    let accepted =
      if (amount <= limit) amount
      else limit;
    let rec_accepted = Cycles.accept(accepted);
    assert (rec_accepted == accepted);
    balance += accepted;
    { accepted = Nat64.fromNat(accepted) };
  };

  public func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };

};
