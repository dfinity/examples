import Array "mo:core/Array";
import Text "mo:core/Text";
import Call "mo:ic/Call";
import IC "mo:ic/Types";

actor SendHttpPost {

  // #region transform
  // Strip HTTP response headers (date, cookies, tracking IDs) that vary across requests.
  // Even in non-replicated mode (used here), the transform is required — the system
  // always invokes it. In replicated mode, stripping non-deterministic fields is
  // essential for consensus to succeed.
  public query func transform({
    context = _context : Blob;
    response : IC.HttpRequestResult;
  }) : async IC.HttpRequestResult {
    { response with headers = [] };
  };
  // #endregion transform

  // #region post_request
  public func send_http_post_request() : async Text {
    let body = "This is a POST request from an ICP canister.".encodeUtf8();

    let request = Call.httpRequest("https://postman-echo.com/post")
      .withMethod(#post)
      // max_response_bytes bounds the response. A larger one fails the call.
      // Under pay-as-you-go pricing it no longer sets the price, because only
      // the bytes that actually arrive are charged.
      .withMaxResponseBytes(3_000)
      .withHeader("Content-Type", "text/plain")
      .withBody(body)
      .withTransform({ function = transform; context = ([] : [Nat8]).toBlob() })
      // Unset expectations are reserved at their worst case: a 60 second round
      // trip, and a transform running to the full query instruction limit.
      // Declaring what this call expects holds far fewer cycles while it runs.
      // The charge is unchanged, because only the resources actually used are
      // billed. Leave headroom, since the reservation doubles as each node's
      // budget for the call.
      .withExpectedRoundtripTimeMs(10_000)
      .withExpectedTransformInstructions(1_000_000)
      // Non-replicated: only one node sends the request. In replicated mode
      // (the builder's default) add an Idempotency-Key header, so the server can
      // deduplicate the request that each node sends independently.
      .nonReplicated();

    // send() computes the cycles the call may need and attaches them.
    let response = await request.send();

    // postman-echo.com echoes back the request data as JSON, letting you
    // verify the POST body and headers were sent correctly.
    switch (response.body.decodeUtf8()) {
      case (?text) text;
      case null "Response is not valid UTF-8";
    };
  };
  // #endregion post_request
};
