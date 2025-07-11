import Blob "mo:base/Blob";
import Text "mo:base/Text";
import IC "ic:aaaaa-aa";

actor {

  //function to transform the response
  public query func transform({
    context : Blob;
    response : IC.http_request_result;
  }) : async IC.http_request_result {
    {
      response with headers = []; // not intersted in the headers
    };
  };

  //PULIC METHOD
  //This method sends a POST request to a URL with a free API we can test.
  public func send_http_post_request() : async Text {

    //1. SETUP ARGUMENTS FOR HTTP GET request

    // 1.1 Setup the URL and its query parameters
    //This URL is used because it allows us to inspect the HTTP request sent from the canister
    let host : Text = "putsreq.com";
    let url = "https://putsreq.com/aL1QS5IbaQd4NTqN3a81"; //HTTP that accepts IPV6

    // 1.2 prepare headers for the system http_request call

    //idempotency keys should be unique so we create a function that generates them.
    let idempotency_key : Text = generateUUID();
    let request_headers = [
      { name = "User-Agent"; value = "http_post_sample" },
      { name = "Content-Type"; value = "application/json" },
      { name = "Idempotency-Key"; value = idempotency_key },
    ];

    // The request body is a Blob, so we do the following:
    // 1. Write a JSON string
    // 2. Convert Text into a Blob
    let request_body_json : Text = "{ \"name\" : \"Grogu\", \"force_sensitive\" : \"true\" }";
    let request_body = Text.encodeUtf8(request_body_json);

    // 1.3 The HTTP request
    let http_request : IC.http_request_args = {
      url = url;
      max_response_bytes = null; //optional for request
      headers = request_headers;
      //note: type of `body` is ?Blob so we pass it here as "?request_body" instead of "request_body"
      body = ?request_body;
      method = #post;
      transform = ?{
        function = transform;
        context = Blob.fromArray([]);
      };
      // Toggle this flag to switch between replicated and non-replicated http outcalls.
      is_replicated = ?false;
    };

    //2. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE, BUT MAKE SURE TO ADD CYCLES.
    let http_response : IC.http_request_result = await (with cycles = 230_949_972_000) IC.http_request(http_request);

    //3. DECODE THE RESPONSE

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

    //4. RETURN RESPONSE OF THE BODY
    let result : Text = decoded_text # ". See more info of the request sent at: " # url # "/inspect";
    result;
  };

  //PRIVATE HELPER FUNCTION
  //Helper method that generates a Universally Unique Identifier
  //this method is used for the Idempotency Key used in the request headers of the POST request.
  //For the purposes of this exercise, it returns a constant, but in practice it should return unique identifiers
  func generateUUID() : Text {
    "UUID-123456789";
  };
};
