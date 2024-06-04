import Random "mo:base/Random";
import Array "mo:base/Array";
import List "mo:base/List";
import Stack "mo:base/Stack";
import Iter "mo:base/Iter";
import Blob "mo:base/Blob";
import Nat "mo:base/Nat";
import Debug "mo:base/Debug";

/// Generate a random maze using cryptographic randomness.
///
/// Illustrates library `Random.mo` for cryptographic randomness. In particlar:
///
/// * asynchronous requests for initial and additional entropy using
///   shared function `Random.blob()`; and
/// * generating bounded, discrete random numbers using class `Random.Finite()`.

actor {

  type Maze = [[var Nat8]];

  let hall : Nat8 = 0;
  let wall : Nat8 = 1;

  func visit(n : Nat8) : Nat8 {
    n | 2
  };

  func visited(n : Nat8) : Bool {
    n & 2 == 2
  };

  func bit(b : Bool) : Nat {
    if (b) 1 else 0;
  };

  /// Given finite source of randomness `f`,
  /// return an optional random number between [0..`max`)
  /// (using rejection sampling).
  /// Return of `null` indicates `f` is exhausted and should be replaced.
  func chooseMax(f : Random.Finite, max : Nat) : ? Nat {
    assert max > 0;
    do ? {
      var n = max - 1 : Nat;
      var k = 0;
      while (n != 0) {
        k *= 2;
        k += bit(f.coin()!);
        n /= 2;
      };
      if (k < max) k else chooseMax(f, max)!;
    };
  };

  func unvisited(i : Nat, j : Nat, m : Maze) : List.List<(Nat,Nat)> {
    let max: Nat = m.size() - 1;
    var cs = List.nil<(Nat,Nat)>();
    if (i > 1 and not visited(m[i - 2][j]))
      // The <(Nat,Nat)> type annotation is not required, but it can slience the underflow warning for i - 2
      cs := List.push<(Nat,Nat)>((i - 2, j), cs);
    if (i + 1 < max and not visited(m[i + 2][j]))
      cs := List.push((i + 2, j), cs);
    if (j > 1 and not visited(m[i][j - 2]))
      cs := List.push<(Nat,Nat)>((i, j - 2), cs);
    if (j + 1 < max and not visited(m[i][j + 2]))
      cs := List.push((i, j + 2), cs);
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
    let m = Array.tabulate<[var Nat8]>(2 * n + 1,
      func i { Array.init(2 * n + 1, wall) });

    // Use iterative depth-first search on odd numbered entries
    // to turn walls into cells connected by random halls
    let s = Stack.Stack<(Nat,Nat)>();
    let entropy = await Random.blob(); // get initial entropy
    var f = Random.Finite(entropy);

    m[0][1] := hall; // Entrance
    m[2*n][2*n-1] := hall; // Exit

    m[1][1] := visit(hall);
    s.push((1, 1));
    loop {
      switch (s.pop()) {
        case null return toText(m);
        case (?(i, j)) {
          let us = unvisited(i, j, m);
          if (not List.isNil(us)) {
            switch (chooseMax(f, List.size(us))) {
              case (? k) {
                s.push((i, j));
                let ? (i1, j1) = List.get(us, k);
                // connect cell (i, j) and (i1, j1)
                m[if (i == i1) i else (Nat.min(i, i1) + 1)]
                  [if (j == j1) j else (Nat.min(j, j1) + 1)] := hall;
                m[i1][j1] := visit(hall);
                s.push((i1, j1));
              };
              case null { // not enough entropy
                Debug.print("need more entropy...");
                let entropy = await Random.blob(); // get more entropy
                f := Random.Finite(entropy);
                s.push((i,j)); // continue from (i,j)
              }
            }
          }
        }
      }
    }
  };

};
