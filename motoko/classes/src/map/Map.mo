import Array "mo:base/Array";
import Cycles "mo:base/ExperimentalCycles";
import Buckets "Buckets";

actor Map {

  let n = 4; // number of buckets

  // divide initial balance amongst self and buckets
  let cycleShare = Cycles.balance() / (n + 1);

  type Key = Nat;
  type Value = Text;

  type Bucket = Buckets.Bucket;

  let buckets : [var ?Bucket] = Array.init(n, null);

  public func get(k : Key) : async ?Value {
    switch (buckets[k % n]) {
      case null null;
      case (?bucket) await bucket.get(k);
    };
  };

  public func put(k : Key, v : Value) : async () {
    let i = k % n;
    let bucket = switch (buckets[i]) {
      case null {
        // provision next send, i.e. Bucket(n, i), with cycles
        Cycles.add(cycleShare);
        let b = await Buckets.Bucket(n, i); // dynamically install a new Bucket
        buckets[i] := ?b;
        b;
      };
      case (?bucket) bucket;
    };
    await bucket.put(k, v);
  };

};
