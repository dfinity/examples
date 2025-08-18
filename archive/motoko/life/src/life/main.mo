// local imports
import Random = "Random";
import State = "State";
import Grid = "Grid";

actor Life {

  let state = do {
    let rand = Random.new();
    State.new(64, func (i, j) { rand.next() % 2 == 1 });
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

