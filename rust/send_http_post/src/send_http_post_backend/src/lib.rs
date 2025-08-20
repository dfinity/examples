//1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};
use serde_json::json;

//Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn send_http_post_request() -> String {
    //2. SETUP ARGUMENTS FOR HTTP GET request

    // 2.1 Setup the URL
    let url = "https://putsreq.com/XaTKyDHZ0O04gkgQwBKz";

    // 2.2 prepare headers for the system http_request call
    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "demo_HTTP_POST_canister".to_string(),
        },
        //For the purposes of this exercise, Idempotency-Key" is hard coded, but in practice
        //it should be generated via code and unique to each POST request. Common to create helper methods for this
        HttpHeader {
            name: "Idempotency-Key".to_string(),
            value: "UUID-123456789".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    //HttpRequestArgs has the following types:
    // pub struct HttpRequestArgs{
    //     pub url: String,
    //     pub max_response_bytes: Option<u64>,
    //     pub method: HttpMethod,
    //     pub headers: Vec<HttpHeader>,
    //     pub body: Option<Vec<u8>>,
    //     pub transform: Option<TransformContext>,
    // }
    //see: https://docs.rs/ic-cdk/latest/ic_cdk/management_canister/struct.HttpRequestArgs.html

    //Where "HttpMethod" has structure:
    // pub enum HttpMethod {
    //     GET,
    //     POST,
    //     HEAD,
    // }
    //See: https://docs.rs/ic-cdk/latest/ic_cdk/management_canister/enum.HttpMethod.html

    //Since the body in HTTP request has type Option<Vec<u8>> it needs to look something like this: Some(vec![104, 101, 108, 108, 111]) ("hello" in ASCII)
    //where the vector of u8s are the UTF. In order to send JSON via POST we do the following:
    //1. Declare a JSON string to send
    //2. Convert that JSON string to array of UTF8 (u8)
    //3. Wrap that array in an optional

    let json_body = json!(
        {
            "name": "Grogu",
            "force_sensitive": true
        }
    );
    let request_body: Option<Vec<u8>> = Some(serde_json::to_vec(&json_body).unwrap());

    let request = HttpRequestArgs {
        url: url.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
        // transform: None, //optional for request
    };

    //3. MAKE HTTPS REQUEST AND WAIT FOR RESPONSE

    //Note: in Rust, `http_request()` already sends the cycles needed
    //so no need for explicit Cycles.add() as in Motoko
    match http_request(&request).await {
        //4. DECODE AND RETURN THE RESPONSE

        //See: https://docs.rs/ic-cdk/latest/ic_cdk/management_canister/struct.HttpRequestResult.html
        Ok(response) => {
            //if successful, `HttpRequestResult` has this structure:
            // pub struct HttpRequestResult{
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //We need to decode that Vec<u8> that is the body into readable text.
            //To do this, we:
            //  1. Call `String::from_utf8()` on response.body
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::debug_print(format!("{:?}", str_body));

            //The API response will looks like this:
            // Hello Grogu

            //Return the body as a string and end the method
            let result: String = format!(
                "{}. See more info of the request sent at: {}/inspect",
                str_body, url
            );
            result
        }
        Err(error) => {
            //Return the error as a string and end the method
            error.to_string()
        }
    }
}

// Strips all data that is not needed from the original response.
#[ic_cdk::query]
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
