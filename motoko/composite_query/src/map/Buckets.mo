import Nat "mo:base/Nat";
import Map "mo:base/RBTree";

persistent actor class Bucket(n : Nat, i : Nat) {

  type Key = Nat;
  type Value = Text;

  transient let map = Map.RBTree<Key, Value>(Nat.compare);

  public query func get(k : Key) : async ?Value {
    assert ((k % n) == i);
    map.get(k);
  };

  public func put(k : Key, v : Value) : async () {
    assert ((k % n) == i);
    map.put(k, v);
  };

};
