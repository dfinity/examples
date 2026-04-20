import Map "mo:core/Map";
import Nat "mo:core/Nat";

persistent actor class Bucket(n : Nat, i : Nat) {

  type Key = Nat;
  type Value = Text;

  let map = Map.empty<Key, Value>();

  public func get(k : Key) : async ?Value {
    assert ((k % n) == i);
    map.get(k);
  };

  public func put(k : Key, v : Value) : async () {
    assert ((k % n) == i);
    map.add(k, v);
  };

};
