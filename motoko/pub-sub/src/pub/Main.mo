// Publisher

import Buffer "mo:base/Buffer";

actor Publisher {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type Subscriber = {
    topic : Text;
    callback : shared Counter -> ();
  };

  let subscribers = Buffer.Buffer<Subscriber>(0);

  public func subscribe(subscriber : Subscriber) {
    subscribers.add(subscriber);
  };

  public func publish(counter : Counter) {
    for (subscriber in subscribers.vals()) {
      if (subscriber.topic == counter.topic) {
        subscriber.callback(counter);
      };
    };
  };
};
