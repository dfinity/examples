import Debug "mo:base/Debug";
import Map "canister:map";

actor Test {

  public func run() : async () {
    var i = 0;
    while (i < 16) {
      let t = debug_show(i);
      assert (null == (await Map.get(i)));
      Debug.print("putting: " # debug_show(i, t));
      await Map.put(i, t);
      assert (?t == (await Map.get(i)));
      i += 1;
    };
  };

};
