import Iter "mo:base/Iter";

actor FavoriteCities {

  // Say hello from the given cities.
  public query func hello_simple(cities : [Text]) : async Text {
    return "Hello from " # debug_show(cities);
  };

  // Say hello from the given cities.
  public query func hello_pretty(cities : [Text]) : async Text {
    let n = cities.size();
    var accum = "";
    switch n {
      case 0 accum #= "nowhere";
      case 1 accum #= cities[0];
      case 2 accum #= cities[0] # " and " # cities[1];
      case _ do {
        for (i in Iter.range(0, n - 2)) {
          accum #= cities[i] # ", ";
        };
        accum #= "and " # cities[n - 1];
      }
    };
    return "Hello from " # accum # "!";
  }
};
