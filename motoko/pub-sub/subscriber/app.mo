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

  // icp-cli automatically injects PUBLIC_CANISTER_ID:publisher into every
  // canister in the project during `icp deploy`.  Read it at runtime via
  // Runtime.envVar so no canister ID is hardcoded or passed as an argument.
  func publisher() : PublisherActor {
    let ?id = Runtime.envVar("PUBLIC_CANISTER_ID:publisher") else
      Runtime.trap("PUBLIC_CANISTER_ID:publisher not set — run icp deploy");
    actor(id) : PublisherActor;
  };

  /// Subscribe to `topic` on the publisher canister.
  public func subscribe(topic : Text) : async () {
    publisher().subscribe({
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
