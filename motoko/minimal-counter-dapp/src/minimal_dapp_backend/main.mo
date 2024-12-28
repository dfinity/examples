actor {

  stable var counter : Nat = 0;

  public func increment() : async Nat {
    counter += 1;
    return counter;
  };

  public func decrement() : async Nat {
    // avoid trap due to Natural subtraction underflow
    if(counter != 0) {
      counter -= 1;
    };
    return counter;
  };

  public query func getCount() : async Nat {
    return counter;
  };

  public func reset() : async Nat {
    counter := 0;
    return counter;
  };
};
