// Subscriber

import Publisher "canister:pub";

type Counter = { topic: Text; value: Nat; };

actor Subscriber {
    let counter_topic = "Apples";
    var count: Nat = 0;

    public func init() {
        Publisher.subscribe({ topic = counter_topic; callback = updateCount; });
    };

    public func updateCount(counter: Counter) {
        count += counter.value;
    };

    public query func getCount(): async Nat {
        count
    };
};
