import Array "mo:base/Array";
import Order "mo:base/Order";
import Int "mo:base/Int";

module Quicksort {

  type Order = Order.Order;

  // Sort the elements of an array using the given comparison function.
  public func sortBy<X>(xs : [X], f : (X, X) -> Order) : [X] {
    let n = xs.size();
    if (n < 2) {
      return xs;
    } else {
      let result = Array.thaw<X>(xs);
      sortByHelper<X>(result, 0, n - 1, f);
      return Array.freeze<X>(result);
    };
  };

  private func sortByHelper<X>(
    xs : [var X],
    l : Int,
    r : Int,
    f : (X, X) -> Order,
  ) {
    if (l < r) {
      var i = l;
      var j = r;
      var swap  = xs[0];
      let pivot = xs[Int.abs(l + r) / 2];
      while (i <= j) {
        while (Order.isLess(f(xs[Int.abs(i)], pivot))) {
          i += 1;
        };
        while (Order.isGreater(f(xs[Int.abs(j)], pivot))) {
          j -= 1;
        };
        if (i <= j) {
          swap := xs[Int.abs(i)];
          xs[Int.abs(i)] := xs[Int.abs(j)];
          xs[Int.abs(j)] := swap;
          i += 1;
          j -= 1;
        };
      };
      if (l < j) {
        sortByHelper<X>(xs, l, j, f);
      };
      if (i < r) {
        sortByHelper<X>(xs, i, r, f);
      };
    };
  };
};
