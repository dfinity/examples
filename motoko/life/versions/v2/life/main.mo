import Debug = "mo:base/Debug";
// local imports
import Random = "Random";
import State = "State";
import Grid = "Grid";

actor Life {

  stable var state : State.State =
    do {
      let rand = Random.new();
      State.new(64, func (i, j) { rand.next() % 2 == 1 });
    };

  system func preupgrade() {
    state := cur.toState();
  };

  system func postupgrade() {
    Debug.print("upgraded to v2!");
  };

  public query func stableState() : async Text {
    debug_show(cur.toState());
  };

  var cur = Grid.Grid(state);

  var nxt = Grid.Grid(State.new(cur.size(), func (i, j) { false; }));

  public func next() : async Text {
    cur.next(nxt);
    let temp = cur;
    cur := nxt;
    nxt := temp;
    cur.toText();
  };

  public query func current() : async Text {
    cur.toText()
  };

};
