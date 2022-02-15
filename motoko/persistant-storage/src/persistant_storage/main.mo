actor {

    stable var counter : Nat = 0;

    public func increment() : async Nat {
        counter += 1;
        return counter;
    };

    public query func get() : async Nat {
        return counter;
    };
    
    public func reset() : async Nat {
        counter := 0;
        return counter;
    };
};
