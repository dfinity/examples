/// Extended actor class used for upgrade/reinstall demonstrations.
/// Includes additional `substractFromValue` endpoint to show new functionality after upgrade.
persistent actor class AnotherChild() {
  let value : Nat = 42;
  public func getValue() : async Nat {
    return value;
  };
  public func addToValue(x : Nat) : async Nat {
    return value + x;
  };
  public func substractFromValue(x : Nat) : async Nat {
    return value - x;
  };
};
