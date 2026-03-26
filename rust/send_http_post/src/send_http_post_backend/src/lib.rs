use ic_cdk::{
    api::canister_self,
    management_canister::{
        http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
        TransformContext, TransformFunc,
    },
};

// #region transform
// Strip HTTP response headers (date, cookies, tracking IDs) that vary across requests.
// Even in non-replicated mode (used here), the transform is required — the system
// always invokes it. In replicated mode, stripping non-deterministic fields is
// essential for consensus to succeed.
#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    HttpRequestResult {
        headers: vec![],
        ..raw.response
    }
}
// #endregion transform

// #region post_request
#[ic_cdk::update]
async fn send_http_post_request() -> String {
    let body = "This is a POST request from an ICP canister.";

    let request = HttpRequestArgs {
        url: "https://postman-echo.com/post".to_string(),
        method: HttpMethod::POST,
        // Always set max_response_bytes to a tight bound. The cycle cost scales
        // with this value, not the actual response size. If omitted, the system
        // assumes 2MB. Unused cycles are refunded, but you still pay for the
        // declared maximum.
        max_response_bytes: Some(3_000),
        headers: vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "text/plain".to_string(),
        }],
        body: Some(body.as_bytes().to_vec()),
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
        // Non-replicated: only one replica sends the request. For replicated
        // mode (true), add an Idempotency-Key header so the server can
        // deduplicate the requests sent by each replica independently.
        is_replicated: Some(false),
    };

    // http_request auto-calculates and attaches the required cycles
    match http_request(&request).await {
        // postman-echo.com echoes back the request data as JSON, letting you
        // verify the POST body and headers were sent correctly.
        Ok(response) => String::from_utf8(response.body).unwrap_or_default(),
        Err(err) => format!("Outcall failed: {err}"),
    }
}
// #endregion post_request
