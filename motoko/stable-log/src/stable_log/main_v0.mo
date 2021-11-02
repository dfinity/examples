import Seq "mo:sequence/Sequence";
import Types "Types";
import Common "Common";
import StableMemory "StableMemory";

actor {
  public type Entry = Types.Entry;

  /// to do --
  /// This actor represents the same log in stable memory
  /// and its tests will ensure that they always match.
  stable var log : Types.Entries = #empty;

  let append = Seq.defaultAppend();

  public func put(text : Text) : async Nat {
    let entry = Common.entry(text);
    let index = StableMemory.put(entry);
    let size = Seq.size(log);
    assert(index == size);
    log := append(log, Seq.make(entry));
    size
  };

  public query func get(index : Nat) : async ?Entry {
    let entry1 = StableMemory.get(index);
    let entry2 = Seq.get(log, index);
    assert entry1 == entry2;
    entry1
  };
};
