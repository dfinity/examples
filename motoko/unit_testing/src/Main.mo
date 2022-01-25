import U "./Utils";
import T "./Types";

actor {
  public func sum (num1 : Nat, num2 : Nat) : async Nat {
    U.sum(num1, num2);
  };
}
