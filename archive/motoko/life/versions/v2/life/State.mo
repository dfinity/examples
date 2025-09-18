import Array "mo:base/Array";
import Iter "mo:base/Iter";
import Nat64 "mo:base/Nat64";

module {

  public type Cell = Bool;

  public type State = {
    #v1 : [[var Cell]];
    #v2 : {size : Nat; bits : [var Nat64]}
  };

  public func readBit(bits : [var Nat64], index : Nat) : Bool {
    let bit = Nat64.fromNat(index);
    let mask : Nat64 = 1 << (bit % 64);
    (bits[Nat64.toNat(bit >> 6)] & mask) == mask
  };

  public func writeBit(bits : [var Nat64], index : Nat, v : Bool) {
    let bit = Nat64.fromNat(index);
    let mask : Nat64 = 1 << (bit % 64);
    let i = Nat64.toNat(bit >> 6);
    if v {
      bits[i] |= mask
    }
    else {
      bits[i] &= ^mask;
    }
  };

  public func new(size : Nat, f : (i : Nat, j : Nat) -> Cell) :
    {#v2 : {size : Nat; bits : [var Nat64]}} {
    let words = (size * size) / 64 + 1;
    let bits = Array.init<Nat64>(words, 0);
    for (i in Iter.range(0, size - 1)) {
      for (j in Iter.range(0, size - 1)) {
        writeBit(bits, i * size + j, f(i, j));
      }
    };
    #v2 {size = size; bits = bits}
  }
}
