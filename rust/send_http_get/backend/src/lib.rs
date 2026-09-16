use ic_cdk_management_canister::{
    transform_context_from_query, HttpMethod, HttpRequest, HttpRequestResult, TransformArgs,
};

// #region transform
// Strip HTTP response headers (date, cookies, tracking IDs) that vary across replicas.
// In replicated mode, all replicas must see an identical response for consensus to
// succeed. The transform ensures this by discarding non-deterministic fields.
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
    // Replicated mode is the builder's default: all subnet nodes make the request
    // independently, providing strong integrity guarantees via consensus.
    let request = HttpRequest::new("https://postman-echo.com/get?greeting=hello-from-icp")
        .with_method(HttpMethod::GET)
        // max_response_bytes bounds the response. A larger one fails the call.
        // Under pay-as-you-go pricing it no longer sets the price, because only
        // the bytes that actually arrive are charged.
        .with_max_response_bytes(3_000)
        .with_header("User-Agent", "ic-canister")
        .with_transform(transform_context_from_query("transform".to_string(), vec![]))
        // Unset expectations are reserved at their worst case: a 60 second round
        // trip, and a transform running to the full query instruction limit.
        // Declaring what this call expects holds far fewer cycles while it runs.
        // The charge is unchanged, because only the resources actually used are
        // billed. Leave headroom, since the reservation doubles as each node's
        // budget for the call.
        .with_expected_roundtrip_time_ms(10_000)
        .with_expected_transform_instructions(1_000_000);

    // send() computes the cycles the call may need and attaches them.
    match request.send().await {
        // postman-echo.com echoes back the request metadata as JSON, letting you
        // verify the query params and headers were sent correctly.
        Ok(response) => String::from_utf8(response.body).unwrap_or_default(),
        Err(err) => format!("Outcall failed: {err}"),
    }
}
// #endregion get_request

ic_cdk::export_candid!();
