actor {
    public shared query (msg) func whoami() : async Principal {
        msg.caller
    };
};
