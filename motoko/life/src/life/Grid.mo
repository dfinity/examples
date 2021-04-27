import State "State";

module {

  public class Grid(state : State.State) {

    let grid = state;

    let n = grid.size();

    public func size() : Nat { n };

    public func get(i : Nat, j : Nat) : State.Cell { grid[i][j] };

    public func set(i : Nat, j : Nat, v : State.Cell) { grid[i][j] := v };

    func count(i : Nat, j : Nat) : Nat { if (grid[i][j]) 1 else 0 };

    func pred(i : Nat) : Nat { (n + i - 1) % n };
    func succ(i : Nat) : Nat { (i + 1) % n };

    func living(i : Nat, j : Nat) : Nat {
      count(pred i, pred j) + count(pred i, j) + count(pred i, succ j) +
      count(     i, pred j)                    + count(     i, succ j) +
      count(succ i, pred j) + count(succ i, j) + count(succ i, succ j)
    };

    func nextCell(i : Nat, j : Nat) : State.Cell {
      let l : Nat = living(i, j);
      if (get(i, j))
        l == 2 or l == 3
      else
        l == 3;
    };

    public func next(dst : Grid) {
      for (i in grid.keys()) {
        for (j in grid[i].keys()) {
          dst.set(i, j, nextCell(i, j));
        };
      };
    };

    public func toText() : Text {
      var t = "";
      for (i in grid.keys()) {
        for (j in grid[i].keys()) {
          t #= if (get(i, j)) "0" else " ";
        };
        if (i + 1 < n) {
	  t #= "\n";
	}
      };
      t
    };
  };
}
