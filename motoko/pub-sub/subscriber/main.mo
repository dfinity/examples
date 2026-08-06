// Subscriber

import Publisher "canister:publisher";

actor Subscriber {

  type Counter = {
    topic : Text;
    value : Nat;
  };

  var count : Nat = 0;

  // The publisher is imported by name via `canister:publisher`. icp-cli injects
  // its principal at deploy time; the mops.toml `--actor-env-alias` flag wires
  // the `publisher` import to the PUBLIC_CANISTER_ID:publisher env var and the
  // publisher's Candid interface (publisher/publisher.did), so no actor type is
  // declared here.
  public func subscribe(topic : Text) : async () {
    await Publisher.subscribe({
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
