use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};

// #region transform
// Strip HTTP response headers (date, cookies, tracking IDs) that vary across replicas.
// In replicated mode, all replicas must see an identical response for consensus to
// succeed — the transform ensures this by discarding non-deterministic fields.
#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    HttpRequestResult {
        headers: vec![],
        ..raw.response
    }
}
// #endregion transform

// #region get_request
#[ic_cdk::update]
async fn send_http_get_request() -> String {
    let request = HttpRequestArgs {
        url: "https://postman-echo.com/get?greeting=hello-from-icp".to_string(),
        method: HttpMethod::GET,
        // Always set max_response_bytes to a tight bound. The cycle cost scales
        // with this value, not the actual response size. If omitted, the system
        // assumes 2MB. Unused cycles are refunded, but you still pay for the
        // declared maximum.
        max_response_bytes: Some(3_000),
        headers: vec![HttpHeader {
            name: "User-Agent".to_string(),
            value: "ic-canister".to_string(),
        }],
        body: None,
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
        // Replicated mode: all subnet nodes make the request independently,
        // providing strong integrity guarantees via consensus.
        is_replicated: Some(true),
    };

    // http_request auto-calculates and attaches the required cycles
    match http_request(&request).await {
        // postman-echo.com echoes back the request metadata as JSON, letting you
        // verify the query params and headers were sent correctly.
        Ok(response) => String::from_utf8(response.body).unwrap_or_default(),
        Err(err) => format!("Outcall failed: {err}"),
    }
}
// #endregion get_request
