import Error "mo:base/Error";
import Types "Types";
import Prim "mo:⛔";
import Principal "mo:base/Principal";
import Cycles "mo:base/ExperimentalCycles";


actor class ExchangeRate() = this {
    type IC = actor {
        http_request : Types.CanisterHttpRequestArgs -> async Types.CanisterHttpResponsePayload;
    };

    let ic : IC = actor("aaaaa-aa");

    public query func transform(raw: Types.CanisterHttpResponsePayload) : async Types.CanisterHttpResponsePayload {
        let transformed: Types.CanisterHttpResponsePayload = {
            status = raw.status;
            body = raw.body;
            headers = [ { name = "abc"; value = "def"; } ];
        };
        transformed
    };

    public shared (msg) func call_http(url: Text) : async { #Ok : { response: Types.CanisterHttpResponsePayload }; #Err : Text } {
        let request: Types.CanisterHttpRequestArgs = {
            url = url;
            max_response_bytes = null;
            headers = [];
            body = null;
            http_method = #GET;
            transform_method_name = ?"transform";
        };
        try {
            Cycles.add(300_000_000_000);
            let response = await ic.http_request(request);
            #Ok({response})
        } catch (err) {
            #Err(Error.message(err))
        }
    };
}
