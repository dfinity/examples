import Array "mo:core/Array";
import Text "mo:core/Text";
import Call "mo:ic/Call";
import IC "mo:ic/Types";

actor SendHttpGet {

  // #region transform
  // Strip HTTP response headers (date, cookies, tracking IDs) that vary across replicas.
  // In replicated mode, all replicas must see an identical response for consensus to
  // succeed — the transform ensures this by discarding non-deterministic fields.
  public query func transform({
    context = _context : Blob;
    response : IC.HttpRequestResult;
  }) : async IC.HttpRequestResult {
    { response with headers = [] };
  };
  // #endregion transform

  // #region get_request
  public func send_http_get_request() : async Text {
    // Replicated mode is the builder's default: all subnet nodes make the request
    // independently, providing strong integrity guarantees via consensus.
    let request = Call.httpRequest("https://postman-echo.com/get?greeting=hello-from-icp")
      // max_response_bytes bounds the response. A larger one fails the call.
      // Under pay-as-you-go pricing it no longer sets the price, because only
      // the bytes that actually arrive are charged.
      .withMaxResponseBytes(3_000)
      .withHeader("User-Agent", "ic-canister")
      .withTransform({ function = transform; context = ([] : [Nat8]).toBlob() })
      // Unset expectations are reserved at their worst case: a 60 second round
      // trip, and a transform running to the full query instruction limit.
      // Declaring what this call expects holds far fewer cycles while it runs.
      // The charge is unchanged, because only the resources actually used are
      // billed. Leave headroom, since the reservation doubles as each node's
      // budget for the call.
      .withExpectedRoundtripTimeMs(10_000)
      .withExpectedTransformInstructions(1_000_000);

    // send() computes the cycles the call may need and attaches them.
    let response = await request.send();

    // postman-echo.com echoes back the request metadata as JSON, letting you
    // verify the query params and headers were sent correctly.
    switch (response.body.decodeUtf8()) {
      case (?text) text;
      case null "Response is not valid UTF-8";
    };
  };
  // #endregion get_request
};
