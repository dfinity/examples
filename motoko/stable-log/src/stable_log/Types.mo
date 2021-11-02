import Seq "mo:sequence/Sequence";

module {

public type Entry = {
  time : Int;
  text : Text;
};

public type Entries = Seq.Sequence<Entry>;

/// Stable-memory representation
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