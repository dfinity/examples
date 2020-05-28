actor Counter {
    var cell : Nat = 0;

    public func increment() : async Nat {
        cell += 1;
        cell
    };

    public func decrement() : async Nat {
        cell -= 1;
        cell
    };
}
