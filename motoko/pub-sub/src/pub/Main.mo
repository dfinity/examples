// Publisher
import List "mo:base/List";

persistent actor Publisher {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type Subscriber = {
    topic : Text;
    callback : shared Counter -> ();
  };

  var subscribers = List.nil<Subscriber>();

  public func subscribe(subscriber : Subscriber) {
    subscribers := List.push(subscriber, subscribers);
  };

  public func publish(counter : Counter) {
    for (subscriber in List.toArray(subscribers).vals()) {
      if (subscriber.topic == counter.topic) {
        subscriber.callback(counter);
      };
    };
  };
}
