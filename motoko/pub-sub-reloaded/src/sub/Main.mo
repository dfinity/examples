// Subscriber

import Publisher "canister:pub";

actor Subscriber {


type NewsMessage = {
    topic : Text;
    content : Text;
    readingTime : Nat;
  };

  var totalReadingTime: Nat = 0;

  public func subscribeToTopic(subscribedTopic : Text) {
    Publisher.subscribe({
      topic = subscribedTopic;
      callback = updateTotalReadingTime;
    });
  };

  public func updateTotalReadingTime(message : NewsMessage) {
    totalReadingTime += message.readingTime;
  };

  public query func getTotalReadingTime() : async Nat {
    totalReadingTime;
  };
}

