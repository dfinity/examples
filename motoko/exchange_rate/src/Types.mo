import M "mo:base/HashMap";


module Types {
    public type Timestamp = Nat64;
    public type Rate = Float;

    public type TimeRange = {
        start : Timestamp;
        end : Timestamp;
    };

    public type RatesWithInterval = {
        interval: Nat8;
        rates: HashMap<Timestamp, Rate>;
    };

    public type HttpHeader = {
        name: Text;
        value: Text;
    };

    public type HttpMethod = {
        GET;
        POST;
        HEAD;
    };

    public type CanisterHttpRequestArgs = {
        url: Text;
        max_response_bytes: ?Nat64;
        headers: [HttpHeader];
        body: ?[Nat8];
        method: HttpMethod;
        transform: ?TransformType;
    };

    public type CanisterHttpResponsePayload = {
        status: Nat128;
        headers: [HttpHeader];
        body: [Nat8];
    };
}
