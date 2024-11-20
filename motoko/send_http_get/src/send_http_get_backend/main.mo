import Blob "mo:base/Blob";
import Cycles "mo:base/ExperimentalCycles";
import Nat64 "mo:base/Nat64";
import Text "mo:base/Text";
import IC "ic:aaaaa-aa";

//Actor
actor {

  //This method sends a GET request to a URL with a free API we can test.
  //This method returns Coinbase data on the exchange rate between USD and ICP
  //for a certain day.
  //The API response looks like this:
  //  [
  //     [
  //         1682978460, <-- start timestamp
  //         5.714, <-- lowest price during time range
  //         5.718, <-- highest price during range
  //         5.714, <-- price at open
  //         5.714, <-- price at close
  //         243.5678 <-- volume of ICP traded
  //     ],
  // ]

  //function to transform the response
  public query func transform({
    context : Blob;
    response : IC.http_request_result;
  }) : async IC.http_request_result {
    {
      response with headers = []; // not intersted in the headers
    };
  };

  public func get_icp_usd_exchange() : async Text {

    //1. SETUP ARGUMENTS FOR HTTP GET request
    let ONE_MINUTE : Nat64 = 60;
    let start_timestamp : Nat64 = 1682978460; //May 1, 2023 22:01:00 GMT
    let host : Text = "api.exchange.coinbase.com";
    let url = "https://" # host # "/products/ICP-USD/candles?start=" # Nat64.toText(start_timestamp) # "&end=" # Nat64.toText(start_timestamp) # "&granularity=" # Nat64.toText(ONE_MINUTE);

    // 1.2 prepare headers for the system http_request call
    let request_headers = [
      { name = "User-Agent"; value = "price-feed" },
    ];

    // 1.3 The HTTP request
    let http_request : IC.http_request_args = {
      url = url;
      max_response_bytes = null; //optional for request
      headers = request_headers;
      body = null; //optional for request
      method = #get;
      transform = ?{
        function = transform;
        context = Blob.fromArray([]);
      };
    };

    //2. ADD CYCLES TO PAY FOR HTTP REQUEST

    //IC management canister will make the HTTP request so it needs cycles
    //See: https://internetcomputer.org/docs/current/motoko/main/cycles

    //The way Cycles.add() works is that it adds those cycles to the next asynchronous call
    //See:
    // - https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-http_request
    // - https://internetcomputer.org/docs/current/references/https-outcalls-how-it-works#pricing
    // - https://internetcomputer.org/docs/current/developer-docs/gas-cost
    Cycles.add<system>(230_949_972_000);

    //3. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE
    let http_response : IC.http_request_result = await IC.http_request(http_request);

    //4. DECODE THE RESPONSE

    //As per the type declarations, the BODY in the HTTP response
    //comes back as Blob. Type signature:

    //public type http_request_result = {
    //     status : Nat;
    //     headers : [HttpHeader];
    //     body : Blob;
    // };

    //We need to decode that Blob that is the body into readable text.
    //To do this, we:
    //  1. Use Text.decodeUtf8() method to convert the Blob to a ?Text optional
    //  2. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
    let decoded_text : Text = switch (Text.decodeUtf8(http_response.body)) {
      case (null) { "No value returned" };
      case (?y) { y };
    };

    //5. RETURN RESPONSE OF THE BODY
    //The API response will looks like this:
    //
    // ("[[1682978460,5.714,5.718,5.714,5.714,243.5678]]")
    //
    //The API response looks like this:
    //  [
    //     [
    //         1682978460, <-- start timestamp
    //         5.714, <-- lowest price during time range
    //         5.718, <-- highest price during range
    //         5.714, <-- price at open
    //         5.714, <-- price at close
    //         243.5678 <-- volume of ICP traded
    //     ],
    // ]
    decoded_text;
  };

};
