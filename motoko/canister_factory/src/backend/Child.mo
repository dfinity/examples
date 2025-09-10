/// Simple actor class with mutable state for demonstrating upgrade vs reinstall behavior.
/// State will be preserved during #upgrade but lost during #reinstall.
persistent actor class Child() {
  let value : Nat = 42;
  public func getValue() : async Nat {
    return value;
  };
  public func addToValue(x : Nat) : async Nat {
    return value + x;
  };
};
