import Blob "mo:core/Blob";
import Text "mo:core/Text";
import IC "ic:aaaaa-aa";

persistent actor {

  // #region transform
  // Strip HTTP response headers (date, cookies, tracking IDs) that vary across requests.
  // Even in non-replicated mode (used here), the transform is required — the system
  // always invokes it. In replicated mode, stripping non-deterministic fields is
  // essential for consensus to succeed.
  public query func transform({
    context : Blob;
    response : IC.http_request_result;
  }) : async IC.http_request_result {
    { response with headers = [] };
  };
  // #endregion transform

  // #region post_request
  public func send_http_post_request() : async Text {
    let body = Text.encodeUtf8("This is a POST request from an ICP canister.");

    let request : IC.http_request_args = {
      url = "https://postman-echo.com/post";
      // Always set max_response_bytes to a tight bound. The cycle cost scales
      // with this value, not the actual response size. If omitted, the system
      // assumes 2MB. Unused cycles are refunded, but you still pay for the
      // declared maximum.
      max_response_bytes = ?(3_000 : Nat64);
      headers = [
        { name = "Content-Type"; value = "text/plain" },
      ];
      body = ?body;
      method = #post;
      transform = ?{ function = transform; context = Blob.fromArray([]) };
      // Non-replicated: only one replica sends the request. For replicated
      // mode (true), add an Idempotency-Key header so the server can
      // deduplicate the requests sent by each replica independently.
      is_replicated = ?false;
    };

    // Cycles must be explicitly attached to management canister calls.
    // The amount is based on request size and max_response_bytes.
    let response = await (with cycles = 230_949_972_000) IC.http_request(request);

    // postman-echo.com echoes back the request data as JSON, letting you
    // verify the POST body and headers were sent correctly.
    switch (Text.decodeUtf8(response.body)) {
      case (?text) text;
      case null "Response is not valid UTF-8";
    };
  };
  // #endregion post_request
};
