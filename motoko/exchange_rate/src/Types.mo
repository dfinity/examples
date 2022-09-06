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
        #get;
        #post;
        #head;
    };

    public type Func = {
        principal: Principal;
        method: Text;
    };

    public type TransformType = {
        #function: Func; 
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
        status: Nat;
        headers: [HttpHeader];
        body: [Nat8];
    };
}
