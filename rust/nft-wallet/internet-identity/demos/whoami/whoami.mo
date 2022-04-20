actor {
    public query ({caller}) func whoami() : async Principal {
        return caller;
    };
};
