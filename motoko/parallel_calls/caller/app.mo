import List "mo:core/List";
import Nat "mo:core/Nat";
import Callee "canister:callee";

actor {

    // The callee is imported by name via `canister:callee`. icp-cli injects its
    // principal at deploy time; the mops.toml `--actor-env-alias` flag wires the
    // `callee` import to the PUBLIC_CANISTER_ID:callee env var and the callee's
    // Candid interface (callee/callee.did), so no actor type is declared here.

    public func sequential_calls(n : Nat) : async Nat {
        var successful_calls = 0;
        for (_ in Nat.range(0, n)) {
            try {
                await Callee.ping();
                successful_calls += 1;
            } catch _ {};
        };
        successful_calls;
    };

    public func parallel_calls(n : Nat) : async Nat {
        let futures = List.empty<async ()>();

        for (_ in Nat.range(0, n)) {
            try {
                futures.add(Callee.ping());
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
