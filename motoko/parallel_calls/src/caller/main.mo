import List "mo:base/List";
import Error "mo:base/Error";
import Principal "mo:base/Principal";
import Iter "mo:base/Iter";

actor {

    type callee_interface = (actor { ping : () -> async () });
    var callee = null : ?callee_interface;

    public func setup_callee(c : Principal) {
        callee := ?(actor (Principal.toText(c)) : callee_interface);
    };

    public func sequential_calls(n : Nat) : async Nat {
        let c = switch (callee) {
            case null {
                throw Error.reject("callee not set up");
            };
            case (?c) { c };
        };

        var successful_calls = 0;

        for (i in Iter.range(1, n)) {
            try {
                await c.ping();
                successful_calls += 1;
            } catch (e) {};
        };
        successful_calls;
    };

    public func parallel_calls(n : Nat) : async Nat {
        let c = switch (callee) {
            case null {
                throw Error.reject("callee not set up");
            };
            case (?c) { c };
        };

        var l = List.nil<async ()>();

        for (i in Iter.range(1, n)) {
            try {
                l := List.push(c.ping(), l);
            } catch (e) {};
        };

        // The responses on the IC will in this example come in the order of the requests in practice.
        // We reverse the list to match the order of the requests here, as the Motoko scheduler has
        // some overhead if the responses are awaited out of order.
        l := List.reverse(l);

        var successful_calls = 0;
        for (a in List.toIter(l)) {
            try {
                await a;
                successful_calls += 1;
            } catch (e) {};
        };

        successful_calls;
    };
};
