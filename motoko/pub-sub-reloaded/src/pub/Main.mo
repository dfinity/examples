// Publisher
import List "mo:base/List";

actor Publisher {

  type NewsMessage = {
    topic : Text;
    content : Text;
    readingTime : Nat;
  };

  type Subscriber = {
    topic : Text;
    callback : shared NewsMessage -> ();
  };

  stable var subscribers = List.nil<Subscriber>();

  public func subscribe(subscriber : Subscriber) {
    subscribers := List.push(subscriber, subscribers);
  };

  public func publish(newsMessage : NewsMessage) {
    for (subscriber in List.toArray(subscribers).vals()) {
      if (subscriber.topic == newsMessage.topic) {
        subscriber.callback(newsMessage);
      };
    };
  };
}
