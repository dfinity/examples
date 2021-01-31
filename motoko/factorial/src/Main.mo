actor Factorial {

  // Calculate the product of all positive integers less than or equal to `n`.
  public query func fac(n : Nat) : async Nat {

    // We implement the recustion in a helper function that does not return
    // asynchronously.
    func go(m : Nat) : Nat {
      if (m == 0) {
        return 1;
      } else {
        return m * go(m - 1);
      };
    };

    // Return.
    return go(n);
  };
};
