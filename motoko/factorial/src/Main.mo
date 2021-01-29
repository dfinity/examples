actor Factorial {

  // Calculate the product of all positive integers less than or equal to `n`.
  public query func fac(n : Nat) : async Nat {
    func go(m : Nat) : Nat {
      if (m == 0) {
        return 1;
      } else {
        return m * go(m - 1);
      }
    };
    return go(n);
  }
}
