actor Counter {

  var counter = 0;

  public query func get() : async Nat {
    counter
  };

  public func set(n: Nat) {
    counter := n
  };

  public func inc() {
    counter += 1
  }

}
