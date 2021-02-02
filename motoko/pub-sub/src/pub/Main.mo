// Publisher

import Array "mo:base/Array";

actor Publisher {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type Subscriber = {
    topic : Text;
    callback : shared Counter -> ();
  };

  var subscribers : [Subscriber] = [];

  public func subscribe(subscriber : Subscriber) {
    subscribers := Array.append<Subscriber>(subscribers, [subscriber]);
  };

  public func publish(counter : Counter) {
    for (subscriber in subscribers.vals()) {
      if (subscriber.topic == counter.topic) {
        subscriber.callback(counter);
      };
    };
  };
};
