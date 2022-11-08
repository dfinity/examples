import HashMap "mo:base/HashMap";
import Principal "mo:base/Principal";

module Types {
    public type Timestamp = Nat64;
    public type Rate = Text;

    public type TimeRange = {
        start : Timestamp;
        end : Timestamp;
    };

    public type RatesWithInterval = {
        interval : Nat64;
        rates : [(Timestamp, Rate)];
    };

    public type HttpHeader = {
        name : Text;
        value : Text;
    };

    public type HttpMethod = {
        #get;
        #post;
        #head;
    };

    public type TransformContext = {
        function : shared query TransformArgs -> async CanisterHttpResponsePayload;
        context : Blob;
    };

    public type CanisterHttpRequestArgs = {
        url : Text;
        max_response_bytes : ?Nat64;
        headers : [HttpHeader];
        body : ?[Nat8];
        method : HttpMethod;
        transform : ?TransformContext;
    };

    public type CanisterHttpResponsePayload = {
        status : Nat;
        headers : [HttpHeader];
        body : [Nat8];
    };

    public type TransformArgs = {
        response : CanisterHttpResponsePayload;
        context : Blob;
    };

    public type IC = actor {
        http_request : Types.CanisterHttpRequestArgs -> async Types.CanisterHttpResponsePayload;
    };
};
