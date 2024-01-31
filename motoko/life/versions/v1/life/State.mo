import Array "mo:base/Array";

module {

  public type Cell = Bool;
  public type State = {
    #v1 : [[var Cell]]
  };

  public func new(size : Nat, f : (i : Nat, j : Nat) -> Cell) : State {
   #v1 (
     Array.tabulate(size, func (i : Nat) : [var Cell] {
        let a : [var Cell] = Array.init(size, false);
        for (j in a.keys()) {
          a[j] := f(i,j);
        };
        a
      }))
  }
}
