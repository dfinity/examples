import Array "mo:core/Array";
import List "mo:core/List";
import Nat "mo:core/Nat";
import Nat32 "mo:core/Nat32";
import Order "mo:core/Order";
import Result "mo:core/Result";
import Text "mo:core/Text";
import Prim "mo:⛔";
import Call "mo:ic/Call";
import IC "mo:ic/Types";

actor SendHttpFlexible {

  /// One distinct answer, and how many nodes returned it.
  public type Tally = {
    value : Text;
    count : Nat;
  };

  public type TimeReport = {
    /// How many nodes answered with a 200.
    responses : Nat;
    /// Each distinct answer, most common first.
    tally : [Tally];
  };

  // #region flexible_request
  public func fetch_server_time() : async Result.Result<TimeReport, Text> {
    // A committee of five, rather than every node on the subnet. Fewer requests
    // cost less and are gentler on the server's rate limits. The tradeoff is
    // that a smaller committee is easier for a single node to skew.
    //
    // `total_requests` cannot exceed the subnet, so the node count is read
    // first. `core` has no wrapper for it, unlike `isReplicatedExecution` and
    // the other system calls in its `InternetComputer` module, so this is one
    // of the few places a canister reaches into `Prim` directly. Rust has it
    // as ordinary public API: `ic_cdk::api::subnet_self_node_count()`.
    let total_requests = Nat32.min(Prim.subnetSelfNodeCount(), 5);
    // Return as soon as a strict majority of the committee has answered.
    let min_responses = total_requests / 2 + 1;

    // No transform function. A flexible outcall hands back each node's own
    // response instead of one the subnet agreed on, so there is nothing to make
    // identical across nodes. Leaving it off also means no cycles are reserved
    // for running it.
    let result = await Call.flexibleHttpRequest("https://postman-echo.com/time/now")
      .withMaxResponseBytes(1_000)
      .withReplication({
        total_requests;
        min_responses;
        max_responses = total_requests;
      })
      .withExpectedRoundtripTimeMs(10_000)
      .send();

    switch (result) {
      // Any count between min_responses and max_responses is a normal success,
      // not a degraded one.
      case (#ok(responses)) #ok(reconcile(responses));
      // global_error says why the replication could not be met: a timeout,
      // out_of_cycles, responses_too_large, or too_many_rejects. node_details
      // reports what the individual nodes did.
      case (#err(err)) #err("Fewer than " # min_responses.toText() # " responses: " # err.message);
    };
  };
  // #endregion flexible_request

  // #region reconcile
  // Reconciling the responses is the canister's job. They do not say which node
  // produced them and their order is not specified, so treat them as an unordered
  // multiset. Each one carries its own status, because no node had to agree with
  // any other. This tallies the distinct bodies and puts the most common first.
  func reconcile(responses : [IC.HttpRequestResult]) : TimeReport {
    let tally = List.empty<Tally>();
    var counted = 0;

    for (response in responses.values()) {
      if (response.status == 200) {
        counted += 1;
        let value = switch (response.body.decodeUtf8()) {
          case (?text) text;
          case null "Response is not valid UTF-8";
        };
        switch (tally.findIndex(func(entry : Tally) : Bool = entry.value == value)) {
          case (?i) tally.put(i, { value; count = tally.at(i).count + 1 });
          case null tally.add({ value; count = 1 });
        };
      };
    };

    {
      responses = counted;
      tally = tally.toArray().sort(func(a : Tally, b : Tally) : Order.Order = Nat.compare(b.count, a.count));
    };
  };
  // #endregion reconcile
};
