use ic_cdk_management_canister::{
    transform_context_from_query, HttpMethod, HttpRequest, HttpRequestResult, TransformArgs,
};

// #region transform
// Strip HTTP response headers (date, cookies, tracking IDs) that vary across requests.
// Even in non-replicated mode (used here), the transform is required: the system
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

    let request = HttpRequest::new("https://postman-echo.com/post")
        .with_method(HttpMethod::POST)
        // max_response_bytes bounds the response. A larger one fails the call.
        // Under pay-as-you-go pricing it no longer sets the price, because only
        // the bytes that actually arrive are charged.
        .with_max_response_bytes(3_000)
        .with_header("Content-Type", "text/plain")
        .with_body(body.as_bytes().to_vec())
        .with_transform(transform_context_from_query("transform".to_string(), vec![]))
        // Unset expectations are reserved at their worst case: a 60 second round
        // trip, and a transform running to the full query instruction limit.
        // Declaring what this call expects holds far fewer cycles while it runs.
        // The charge is unchanged, because only the resources actually used are
        // billed. Leave headroom, since the reservation doubles as each node's
        // budget for the call.
        .with_expected_roundtrip_time_ms(10_000)
        .with_expected_transform_instructions(1_000_000)
        // Non-replicated: only one node sends the request. In replicated mode
        // (the builder's default) add an Idempotency-Key header, so the server can
        // deduplicate the request that each node sends independently.
        .non_replicated();

    // send() computes the cycles the call may need and attaches them.
    match request.send().await {
        // postman-echo.com echoes back the request data as JSON, letting you
        // verify the POST body and headers were sent correctly.
        Ok(response) => String::from_utf8(response.body).unwrap_or_default(),
        Err(err) => format!("Outcall failed: {err}"),
    }
}
// #endregion post_request

ic_cdk::export_candid!();
