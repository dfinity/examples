/// Extended counter actor class used for upgrade/reinstall demonstrations.
/// Adds `substractFromValue` to show new functionality introduced after an upgrade.
/// State is preserved during #upgrade but reset to 42 during #reinstall.
persistent actor class CounterV2() {
  var value : Nat = 42;
  public func getValue() : async Nat { value };
  public func addToValue(x : Nat) : async Nat {
    value := value + x;
    value
  };
  public func substractFromValue(x : Nat) : async Nat {
    value := value - x;
    value
  };
};
