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
        interval : Nat8;
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

    public type TransformType = {
        #function : shared CanisterHttpResponsePayload -> async CanisterHttpResponsePayload;
    };

    public type CanisterHttpRequestArgs = {
        url : Text;
        max_response_bytes : ?Nat64;
        headers : [HttpHeader];
        body : ?[Nat8];
        method : HttpMethod;
        transform : ?{
            #function : shared query CanisterHttpResponsePayload -> async CanisterHttpResponsePayload;
        };
    };

    public type CanisterHttpResponsePayload = {
        status : Nat;
        headers : [HttpHeader];
        body : [Nat8];
    };
};
