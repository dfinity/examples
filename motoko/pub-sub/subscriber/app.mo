// Subscriber

actor Subscriber {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type PublisherActor = actor {
    subscribe : shared { topic : Text; callback : shared Counter -> () } -> ();
  };

  var count : Nat = 0;

  // Register with `publisher` for `topic`. The publisher principal is passed
  // at runtime so the subscriber is not hard-wired to a specific canister.
  public func init(publisher : Principal, topic : Text) : async () {
    let p = actor(debug_show publisher) : PublisherActor;
    p.subscribe({
      topic;
      callback = updateCount;
    });
  };

  public func updateCount(counter : Counter) {
    count += counter.value;
  };

  public query func getCount() : async Nat {
    count;
  };
}
