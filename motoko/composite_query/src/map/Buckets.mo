import Nat "mo:core/Nat";
import Map "mo:core/RBTree";

actor class Bucket(n : Nat, i : Nat) {

  type Key = Nat;
  type Value = Text;

  let map = Map.RBTree<Key, Value>(Nat.compare);

  public query func get(k : Key) : async ?Value {
    assert ((k % n) == i);
    map.get(k);
  };

  public func put(k : Key, v : Value) : async () {
    assert ((k % n) == i);
    map.put(k, v);
  };

};
