// Subscriber

import Runtime "mo:core/Runtime";

actor Subscriber {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  type PublisherActor = actor {
    subscribe : shared { topic : Text; callback : shared Counter -> () } -> ();
  };

  var count : Nat = 0;

  /// Subscribe to `topic` on the publisher canister.
  /// The publisher principal is read from PUBLIC_CANISTER_ID:publisher,
  /// which icp-cli injects into every canister during `icp deploy`.
  public func subscribe(topic : Text) : async () {
    let ?id = Runtime.envVar("PUBLIC_CANISTER_ID:publisher") else
      Runtime.trap("PUBLIC_CANISTER_ID:publisher not set — run icp deploy");
    let pub = actor(id) : PublisherActor;
    pub.subscribe({
      topic;
      callback = updateCount;
    });
  };

  // Oneway callback invoked by the publisher when a message is published.
  // Returns () (not async ()) — this is a fire-and-forget call with no reply.
  public func updateCount(counter : Counter) : () {
    count += counter.value;
  };

  public query func getCount() : async Nat {
    count;
  };
}
