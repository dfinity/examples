import Seq "mo:sequence/Sequence";

module {

public type Entry = {
  time : Int;
  text : Text;
};

public type Entries = Seq.Sequence<Entry>;

/// Stable-memory representation.
///
/// This module gives the conceptual design of stable memory, in terms of Motoko types.
///
/// Within this memory, we represent a (conceptual) cartesian tree, like type Entries,
/// except without relying on flexible memory or stable vars for storage, and instead using
/// (experimental) direct access to stable memory.
///
/// The conceptual design of this memory follows the cartesian tree data structure used by [sequence package](https://github.com/matthewhammer/motoko-sequence/).
///
/// In particular, when we insert a new entry, we use a version of [its append algorithm](https://github.com/matthewhammer/motoko-sequence/blob/e57b88cf4aa4852c7f66b9150692e256911c1425/src/Sequence.mo#L79)
///
/// Rather than use GC-allocated memory, however, we represent these allocations as a new
/// path representation  (see type PathRep defined below) within a fresh chunk of stable memory,
/// which we treat as a giant array of such path representations.
///
/// The (probabilistic) complexity analysis of this tree shows that it stay balanced (in expectation)
/// as these new paths are added; so more "re-balancing steps" are unneeded.
/// Consequently, we append new (variable-sized) entries to the log
/// by only appending to stable memory, while still achieving
/// efficient updates and lookups (each in O(log n) time).

public module StableMemory {

  /// Cartesian tree node (idealized; compare to NodeRep).
  public  type Node = {
    level : Nat64;
    size : Nat;
    left : Path;
    right : Path;
  };

  /// Cartesian tree path (idealized; compare to PathRep).
  public type Path = {
    #entry : Entry;
    #node : Node;
    #index : NodeIndex;
    #empty
  };

  /// We treat stable memory as a big growable array,
  /// where each array position holds a path representation.
  public type StableMemory = [PathRep];

  /// Where to find a path in stable memory?
  public type PathIndex = Nat;

  /// A path representation packs its
  /// nodes' representation into an array.
  public type PathRep = {
    nodes : [ NodeRep ];
    entry : Entry;
  };

  /// We index (name) each represented node
  /// Using its path (index/name) and its offset
  /// within the path from the root (offset 0).
  public type NodeIndex = {
    path : PathIndex ;
    offset : Nat
  };

  /// We represent a node's children differently,
  /// either relying on the enclosing path's node array
  /// for the child we call "next", either ?#right or ?#left or null,
  /// or using its "full name" within stable memory, as a ?NodeIndex.
  /// For both cases, null encodes an #empty Path as a PathRep.
  public type NodeRep = {
    level : Nat32;
    size : Nat;
    next : ?{ #left; #right };
    other : ?NodeIndex;
  };
}
}