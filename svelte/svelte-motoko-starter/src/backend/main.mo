import Principal "mo:base/Principal";

persistent actor {
    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };
};