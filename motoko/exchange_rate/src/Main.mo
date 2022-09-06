import Error "mo:base/Error";
import Types "Types";
import Prim "mo:⛔";
import Principal "mo:base/Principal";
import Cycles "mo:base/ExperimentalCycles";


actor ExchangeRate {
    type IC = actor {
        http_request : Types.CanisterHttpRequestArgs -> async Types.CanisterHttpResponsePayload;
    };

    let ic : IC = actor("aaaaa-aa");

    public shared (msg) func call_http(url: Text) : async { #Ok : { response: Types.CanisterHttpResponsePayload }; #Err : Text } {
        let request: Types.CanisterHttpRequestArgs = {
            url = url;
            max_response_bytes = null;
            headers = [];
            body = null;
            method = #get;
            transform = null;
        }; 
        try {
            Cycles.add(310_130_000_000);
            let response = await ic.http_request(request);
            #Ok({response})
        } catch (err) {
            #Err(Error.message(err))
        }
    };
}
