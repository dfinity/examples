import Iter "mo:base/Iter";
import List "mo:base/List";
import Array "mo:base/Array";
import Order "mo:base/Order";

import Text "mo:base/Text";

module BaseTypes {
    public type PublicKey = Text;
    public type Ciphertext = Text;
    public type DeviceAlias = Text;
    public type GetCiphertextError = { #notFound; #notSynced };

    // Helper function that sorts an iterator of pairs by [#left] or [#right] column. 
    //
    // Returns:
    //      Immutable array of sorted pairs
    public func sort_pairs_by_column<A<: Text, B<: Text>(pairs: Iter.Iter<(A, B)>, factor: {#left; #right}): [(A, B)] {
        let mutable_array = List.toVarArray<(A, B)>(Iter.toList(pairs));
        let comparator = switch (factor) {
            case (#left) { 
                func (a: (A, B), b: (A, B)): Order.Order {
                    Text.compare(a.0, b.0)
                }
            };
            case (#right) {
                func (a: (A, B), b: (A, B)): Order.Order {
                    Text.compare(a.1, b.1)
                }
            };
        };
        Array.sortInPlace(mutable_array, comparator);
        Array.freeze(mutable_array)
    };
}