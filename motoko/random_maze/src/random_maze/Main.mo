import Array "mo:core/Array";
import VarArray "mo:core/VarArray";
import PureList "mo:core/pure/List";
import Stack "mo:core/Stack";
import Nat "mo:core/Nat";
import Random "mo:core/Random";

/// Generate a random maze using cryptographic randomness.
///
/// Illustrates library `Random.mo` for cryptographic randomness. In particular:
///
/// * asynchronous entropy via `Random.crypto()`, an `AsyncRandom` that
///   automatically fetches more entropy from the management canister when needed; and
/// * generating bounded discrete random numbers using `await* random.natRange()`.

persistent actor {

  type Maze = [[var Nat8]];

  let hall : Nat8 = 0;
  let wall : Nat8 = 1;

  func visit(n : Nat8) : Nat8 {
    n | 2
  };

  func visited(n : Nat8) : Bool {
    n & 2 == 2
  };

  func unvisited(i : Nat, j : Nat, m : Maze) : PureList.List<(Nat, Nat)> {
    let max : Nat = m.size() - 1;
    var cs = PureList.empty<(Nat, Nat)>();
    if (i > 1 and not visited(m[i - 2][j]))
      // The <(Nat,Nat)> type annotation is not required, but it can silence the underflow warning for i - 2
      cs := cs.pushFront<(Nat, Nat)>((i - 2, j));
    if (i + 1 < max and not visited(m[i + 2][j]))
      cs := cs.pushFront((i + 2, j));
    if (j > 1 and not visited(m[i][j - 2]))
      cs := cs.pushFront<(Nat, Nat)>((i, j - 2));
    if (j + 1 < max and not visited(m[i][j + 2]))
      cs := cs.pushFront((i, j + 2));
    cs;
  };

  func toText(maze : Maze) : Text {
    var t = "\n";
    for (row in maze.vals()) {
      for (col in row.vals()) {
        t #= if (col == wall) "🟥" else "⬜";
      };
      t #= "\n";
    };
    t
  };

  /// Given n, returns a maze of n * n cells,
  /// separated by n + 1 partial walls.
  ///
  /// https://en.wikipedia.org/wiki/Maze_generation_algorithm
  /// https://en.wikipedia.org/wiki/Maze_generation_algorithm#Iterative_implementation
  public func generate(size : Nat) : async Text {

    let n = Nat.max(1, size / 2);

    // Construct a maze of mutable, unvisited walls
    let m = Array.tabulate(2 * n + 1,
      func _ { VarArray.repeat(wall, 2 * n + 1) });

    // Use iterative depth-first search on odd numbered entries
    // to turn walls into cells connected by random halls.
    // AsyncRandom fetches entropy on demand — no need to manage blobs manually.
    let s = Stack.empty<(Nat, Nat)>();
    let random = Random.crypto();

    m[0][1] := hall; // Entrance
    m[2 * n][2 * n - 1] := hall; // Exit

    m[1][1] := visit(hall);
    s.push((1, 1));
    loop {
      switch (s.pop()) {
        case null return toText(m);
        case (?(i, j)) {
          let us = unvisited(i, j, m);
          if (not us.isEmpty()) {
            let k = await* random.natRange(0, us.size());
            s.push((i, j));
            let ?(i1, j1) = us.get(k) else return toText(m);
            // connect cell (i, j) and (i1, j1)
            m[if (i == i1) i else (Nat.min(i, i1) + 1)]
              [if (j == j1) j else (Nat.min(j, j1) + 1)] := hall;
            m[i1][j1] := visit(hall);
            s.push((i1, j1));
          }
        }
      }
    }
  };

};
