import AssocList "mo:stdlib/AssocList";
import Iter "mo:stdlib/Iter";
import List "mo:stdlib/List";
import Option "mo:stdlib/Option";

module {
  type K = Char;

  public class Map<V>() {
    var map = List.nil<(K, V)>();

    func isEq(x: K, y: K): Bool { x == y };

    public func set(key: K, value: V) {
      let (updated, _) = AssocList.replace<K, V>(map, key, isEq, ?value);
      map := updated;
    };

    public func get(key: K): V {
      Option.unwrap<V>(AssocList.find<K, V>(map, key, isEq))
    };

    public func del(key: K) {
      func notKey(key: K): ((K, V)) -> Bool {
        func (item: (K, V)): Bool = {
          let (itemKey, _) = item;
          itemKey != key
        }
      };
      map := List.filter<(K, V)>(map, notKey(key));
    };
  };

  public func fromEntries<V>(arr: [(K, V)]): Map<V> {
    let map = Map<V>();
    for ((key, value) in Iter.fromArray<(K, V)>(arr)) {
      map.set(key, value);
    };
    map
  };
}
