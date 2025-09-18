import Iter "mo:base/Iter";
import State "State";

module {

  public class Grid(state : State.State) {

    let (#v2 ({size = n; bits : [var Nat64]})) =
      switch state {
        case (#v1 css) {
          State.new(css.size(), func (i, j) { css[i][j] })
        };
        case (#v2 state) { #v2 state };
      };

    public func size() : Nat { n };

    public func get(i : Nat, j : Nat) : State.Cell {
      State.readBit(bits, i * n + j);
    };

    public func set(i : Nat, j : Nat, v : State.Cell) {
      State.writeBit(bits, i * n + j, v);
    };

    func pred(i : Nat) : Nat { (n + i - 1) % n };

    func succ(i : Nat) : Nat { (i + 1) % n };

    func count(i : Nat, j : Nat) : Nat { if (get(i, j)) 1 else 0 };

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
      for (i in Iter.range(0, n - 1)) {
        for (j in Iter.range(0, n - 1)) {
          dst.set(i, j, nextCell(i, j));
        };
      };
    };

    public func toText() : Text {
      var t = "";
      for (i in Iter.range(0, n - 1)) {
        for (j in Iter.range(0, n - 1)) {
          t #= if (get(i, j)) "2" else " ";
        };
        if (i + 1 < n) {
	  t #= "\n";
	}
      };
      t
    };

    public func toState() : State.State {
      #v2 {size = n; bits = bits}
    };

  };
}
