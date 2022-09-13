import HashMap "mo:base/HashMap";
import Principal "mo:base/Principal";


module Types {
    public type Timestamp = Nat64;
    public type Rate = Float;

    public type TimeRange = {
        start : Timestamp;
        end : Timestamp;
    };

    public type RatesWithInterval = {
        interval: Nat8;
        rates: HashMap.HashMap<Timestamp, Rate>;
    };

    public type HttpHeader = {
        name: Text;
        value: Text;
    };

    public type HttpMethod = {
        #GET;
        #POST;
        #HEAD;
    };

    public type CanisterHttpRequestArgs = {
        url: Text;
        max_response_bytes: ?Nat64;
        headers: [HttpHeader];
        body: ?[Nat8];
        http_method: HttpMethod;
        transform_method_name: ?Text;
    };

    public type CanisterHttpResponsePayload = {
        status: Nat64;
        headers: [HttpHeader];
        body: [Nat8];
    };
}
