// Publisher
import Array "mo:core/Array";

persistent actor Publisher {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type Subscriber = {
    topic : Text;
    callback : shared Counter -> ();
  };

  var subscribers : [Subscriber] = [];

  public func subscribe(subscriber : Subscriber) : async () {
    subscribers := subscribers.concat([subscriber]);
  };

  public func publish(counter : Counter) : async () {
    for (subscriber in subscribers.vals()) {
      if (subscriber.topic == counter.topic) {
        subscriber.callback(counter);
      };
    };
  };
}
