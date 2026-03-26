import Blob "mo:base/Blob";
import Text "mo:base/Text";
import IC "ic:aaaaa-aa";

persistent actor {

  // #region transform
  // Strip HTTP response headers (date, cookies, tracking IDs) that vary across replicas.
  // In replicated mode, all replicas must see an identical response for consensus to
  // succeed — the transform ensures this by discarding non-deterministic fields.
  public query func transform({
    context : Blob;
    response : IC.http_request_result;
  }) : async IC.http_request_result {
    { response with headers = [] };
  };
  // #endregion transform

  // #region get_request
  public func send_http_get_request() : async Text {
    let request : IC.http_request_args = {
      url = "https://postman-echo.com/get?greeting=hello-from-icp";
      // Always set max_response_bytes to a tight bound. The cycle cost scales
      // with this value, not the actual response size. If omitted, the system
      // assumes 2MB. Unused cycles are refunded, but you still pay for the
      // declared maximum.
      max_response_bytes = ?(3_000 : Nat64);
      headers = [{ name = "User-Agent"; value = "ic-canister" }];
      body = null;
      method = #get;
      transform = ?{ function = transform; context = Blob.fromArray([]) };
      // Replicated mode: all subnet nodes make the request independently,
      // providing strong integrity guarantees via consensus.
      is_replicated = ?true;
    };

    // Cycles must be explicitly attached to management canister calls.
    // The amount is based on request size and max_response_bytes.
    let response = await (with cycles = 230_949_972_000) IC.http_request(request);

    // postman-echo.com echoes back the request metadata as JSON, letting you
    // verify the query params and headers were sent correctly.
    switch (Text.decodeUtf8(response.body)) {
      case (?text) text;
      case null "Response is not valid UTF-8";
    };
  };
  // #endregion get_request
};
