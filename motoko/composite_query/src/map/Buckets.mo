import Nat "mo:core/Nat";
import Map "mo:core/Map";

persistent actor class Bucket(n : Nat, i : Nat) {

  type Key = Nat;
  type Value = Text;

  let map = Map.empty<Key, Value>();

  public query func get(k : Key) : async ?Value {
    assert ((k % n) == i);
    Map.get(map, Nat.compare, k);
  };

  public func put(k : Key, v : Value) : async () {
    assert ((k % n) == i);
    Map.add(map, Nat.compare, k, v);
  };

};
