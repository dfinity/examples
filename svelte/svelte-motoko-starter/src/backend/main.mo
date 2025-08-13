import Principal "mo:core/Principal";

actor {
    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };
};