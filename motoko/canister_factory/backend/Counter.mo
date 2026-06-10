/// Simple counter actor class with mutable state for demonstrating upgrade vs reinstall behavior.
/// State is preserved during #upgrade but reset to 42 during #reinstall.
persistent actor class Counter() {
  var value : Nat = 42;
  public func getValue() : async Nat { value };
  public func addToValue(x : Nat) : async Nat {
    value := value + x;
    value
  };
};
