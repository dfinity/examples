import List "mo:core/List";
import Error "mo:core/Error";
import Principal "mo:core/Principal";
import Nat "mo:core/Nat";

persistent actor {

    type callee_interface = (actor { ping : () -> async () });
    var callee = null : ?callee_interface;

    public func setup_callee(c : Principal) : () {
        callee := ?(actor (c.toText()) : callee_interface);
    };

    public func sequential_calls(n : Nat) : async Nat {
        let c = switch (callee) {
            case null {
                throw Error.reject("callee not set up");
            };
            case (?c) { c };
        };

        var successful_calls = 0;

        for (_ in Nat.range(0, n)) {
            try {
                await c.ping();
                successful_calls += 1;
            } catch _ {};
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

        let l = List.empty<async ()>();

        for (_ in Nat.range(0, n)) {
            try {
                l.add(c.ping());
            } catch _ {};
        };

        // The responses on the IC will in this example come in the order of the requests in practice.
        // We use List.add (append) so the order already matches the request order.
        var successful_calls = 0;
        for (a in l.values()) {
            try {
                await a;
                successful_calls += 1;
            } catch _ {};
        };

        successful_calls;
    };
};
