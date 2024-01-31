import Principal "mo:base/Principal";

actor {
    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };
};