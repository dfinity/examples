use ic_cdk::{api::canister_self, management_canister::{http_request as canister_http_outcall, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs, TransformContext, TransformFunc}};

#[ic_cdk::update]
// A function to get any typicode post, with threshold consensus
async fn get_typicode_post(id: u64) -> Result<String, String> {

    // A transform function is a canister query method that processes the raw HTTP response
    // before it's returned to the main async function.
    //
    // Its primary purpose for replicated calls is to ensure **determinism**. Every replica
    // in the subnet must agree on the exact same final response to reach consensus.
    // HTTP responses often contain non-deterministic data (e.g., 'Date' or 'Server'
    // headers can vary slightly between provider nodes).
    //
    // This function strips that variable data, ensuring the response is identical
    // across all replicas, allowing consensus to be achieved.
    // 
    // In our case, when calling out to typicode, simply dropping all headers is enough to ensure determinism
    let transform = Some(TransformContext {
        function: TransformFunc(candid::Func {
            principal: canister_self(),
            method: "drop_headers".to_string(),
        }),
        context: vec![],
    });

    // Create actual HTTP request:
    let arg = HttpRequestArgs {
        url: format!("https://jsonplaceholder.typicode.com/posts/{}", id),
        // The maximum size (in bytes) of the response before, or after transformation.
        // If "None" is used, it defaults to 2MB.
        max_response_bytes: Some(10000),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        transform,
        // As (after dropping the headers) the typicode response is always the same for a given ID, we want this request to be replicated (and obtain threshold consensus)
        is_replicated: Some(true),
    };

    // Make the actual outcall. 
    canister_http_outcall(&arg)
        .await
        .map_err(|e| e.to_string())
        .map(|response| {
            String::from_utf8(response.body).unwrap_or_else(|_| "Invalid UTF-8".to_string())
        })
}

// Transform query is only used by the http_outcalls, so it doesn't need to be part of the canister's interface
#[ic_cdk::query(hidden=true)]
fn drop_headers(
    args: TransformArgs,
) -> HttpRequestResult { 
    // Drop all headers, keep everything else the same
    HttpRequestResult {
        status: args.response.status,
        body: args.response.body,
        headers: vec![],
    }
}

#[ic_cdk::update]
// A function to generate retrieve the price of Bitcoin in USD. 
// Importantly, such a request is fast moving / non deterministic, meaning a replicated outcall might not reach consensus.
// We therefore showcase how to make a non-replicated outcalls.
async fn get_bitcoin_price() -> Result<String, String> {

    // Create actual HTTP request:
    let arg = HttpRequestArgs {
        url: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
        // The maximum size (in bytes) of the response before, or after transformation.
        // If "None" is used, it defaults to 2MB.
        max_response_bytes: Some(10000),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        // No need to transform the response any longer, as it is not replicated
        transform: None,
        // The request is not replicated. This means that a single node will attempt the outcall. This is ideal for fast moving data (such as eg crypto prices), non idempotent operations and so on.
        is_replicated: Some(false),
    };

    // Make the actual outcall. 
    canister_http_outcall(&arg)
        .await
        .map_err(|e| e.to_string())
        .map(|response| {
            String::from_utf8(response.body).unwrap_or_else(|_| "Invalid UTF-8".to_string())
        })
}

ic_cdk::export_candid!();
