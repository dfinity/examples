actor Factorial {
  public func fac(n: Nat) : async Nat {
    func go(m: Nat) : Nat {
      if (m == 0) {
        1
      } else {
        m * go(m - 1)
      }
    };
    go(n)
  }
}
