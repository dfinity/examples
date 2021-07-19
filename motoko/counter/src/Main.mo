actor Counter {

  stable var counter = 0;

  // Get the value of the counter.
  public query func get() : async Nat {
    return counter;
  };

  // Set the value of the counter.
  public func set(n: Nat) {
    counter := n;
  };

  // Increment the value of the counter.
  public func inc() {
    counter += 1;
  };
};
