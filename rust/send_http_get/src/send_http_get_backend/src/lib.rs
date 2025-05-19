//1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};

//Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_icp_usd_exchange() -> String {
    //2. SETUP ARGUMENTS FOR HTTP GET request

    // 2.1 Setup the URL and its query parameters
    type Timestamp = u64;
    let start_timestamp: Timestamp = 1682978460; //May 1, 2023 22:01:00 GMT
    let seconds_of_time: u64 = 60; //we start with 60 seconds
    let host = "api.exchange.coinbase.com";
    let url = format!(
        "https://{}/products/ICP-USD/candles?start={}&end={}&granularity={}",
        host,
        start_timestamp.to_string(),
        start_timestamp.to_string(),
        seconds_of_time.to_string()
    );

    // 2.2 prepare headers for the system http_request call
    let request_headers = vec![HttpHeader {
        name: "User-Agent".to_string(),
        value: "exchange_rate_canister".to_string(),
    }];

    let request = HttpRequestArgs {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,               //optional for request
        max_response_bytes: None, //optional for request
        // transform: None,          //optional for request
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
        headers: request_headers,
    };

    //3. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE

    //Note: in Rust, `http_request()` already sends the cycles needed
    //so no need for explicit Cycles.add() as in Motoko
    match http_request(&request).await {
        //4. DECODE AND RETURN THE RESPONSE

        //See: https://docs.rs/ic-cdk/latest/ic_cdk/management_canister/struct.HttpRequestResult.html
        Ok(response) => {
            //if successful, `HttpRequestResult` has this structure:
            // pub struct HttpRequestResult {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //We need to decode that Vec<u8> that is the body into readable text.
            //To do this, we call `String::from_utf8()` on response.body
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
            //  ]

            //Return the body as a string
            str_body
        }
        Err(error) => {
            //Return the error as a string
            error.to_string()
        }
    }
}

// Strips all data that is not needed from the original response.
// Read more here https://internetcomputer.org/docs/references/ic-interface-spec#ic-http_request
#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpRequestResult {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == 200u8 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::debug_print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    res
}
