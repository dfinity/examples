//1. DECLARE IC MANAGEMENT CANISTER
//This also includes methods and types needed
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};

//Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_icp_usd_exchange() -> String {

  //2. SETUP ARGUMENTS FOR HTTP GET request

    // 2.1 Setup the URL and its query parameters
    type Timestamp = u64;
    let start_timestamp : Timestamp = 1682978460; //May 1, 2023 22:01:00 GMT
    let seconds_of_time : u64 = 60; //we start with 60 seconds
    let host = "api.pro.coinbase.com";
    let url = format!("https://{}/products/ICP-USD/candles?start={}&end={}&granularity={}", host, start_timestamp.to_string(), start_timestamp.to_string(),seconds_of_time.to_string());

    // 2.2 prepare headers for the system http_request call
    //Note that `HttpHeader` is declared in line 4
    let request_headers = vec![
        HttpHeader { name: "Host".to_string(), value: format!("{}:443",host) }, 
        HttpHeader { name: "User-Agent".to_string(), value: "exchange_rate_canister".to_string() },
    ];

    //note "CanisterHttpRequestArgument" and "HttpMethod" are declared in line 4
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None, //optional for request
        max_response_bytes: None, //optional for request
        transform: None, //optional for request
        headers: request_headers,
    };

    //3. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE

    //Note: in Rust, `http_request()` already sends the cycles needed 
    //so no need for explicit Cycles.add() as in Motoko
    match http_request(request).await {
        
        //4. DECODE AND RETURN THE RESPONSE
    
        //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
        Ok((response,)) => {

            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //We need to decode that Vec<u8> that is the body into readable text. 
            //To do this, we:
            //  1. Call `String::from_utf8()` on response.body
            //  3. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

                //The API response will looks like this:

                // ("[[1682978460,5.714,5.718,5.714,5.714,243.5678]]")

                //Which can be formatted as this
                //  [
                //     [
                //         1682978460, <-- start/timestamp
                //         5.714, <-- low
                //         5.718, <-- high
                //         5.714, <-- open
                //         5.714, <-- close
                //         243.5678 <-- volume
                //     ],

            //Return the body as a string and end the method
            str_body
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            
            //Return the error as a string and end the method
            message
        }
    }
}