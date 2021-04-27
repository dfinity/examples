import Nat = "mo:base/Nat";
import Nat32 = "mo:base/Nat32";

module {
  public func new() : { next : () -> Nat32 } =
    object {
      let modulus = 0x7fffffff;
      var state : Nat32 = 1;

      public func next() : Nat32
      {
        state := Nat32.fromNat(Nat32.toNat(state) * 48271 % modulus);
        state;
      };

    };
};

