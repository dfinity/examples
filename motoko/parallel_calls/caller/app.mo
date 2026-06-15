import List "mo:core/List";
import Nat "mo:core/Nat";
import Runtime "mo:core/Runtime";

actor {

    type CalleeActor = actor { ping : () -> async () };

    // Read the callee principal from PUBLIC_CANISTER_ID:callee, injected by
    // icp-cli during `icp deploy`. The <system> type parameter explicitly
    // declares that this function uses system capability (required by
    // Runtime.envVar and actor()).
    func callee<system>() : CalleeActor {
        let ?id = Runtime.envVar<system>("PUBLIC_CANISTER_ID:callee") else
            Runtime.trap("PUBLIC_CANISTER_ID:callee not set — run icp deploy");
        actor(id) : CalleeActor;
    };

    public func sequential_calls(n : Nat) : async Nat {
        let c = callee<system>();
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
        let c = callee<system>();
        let futures = List.empty<async ()>();

        for (_ in Nat.range(0, n)) {
            try {
                futures.add(c.ping());
            } catch _ {};
        };

        var successful_calls = 0;
        for (f in futures.values()) {
            try {
                await f;
                successful_calls += 1;
            } catch _ {};
        };
        successful_calls;
    };
};
