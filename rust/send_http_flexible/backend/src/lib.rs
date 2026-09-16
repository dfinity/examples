use candid::{CandidType, Nat};
use ic_cdk::api::subnet_self_node_count;
use ic_cdk_management_canister::{
    FlexibleHttpRequest, FlexibleHttpRequestResult, HttpRequestResult, ReplicationCounts,
};

/// One distinct answer, and how many nodes returned it.
#[derive(CandidType)]
struct Tally {
    value: String,
    count: u32,
}

#[derive(CandidType)]
struct TimeReport {
    /// How many nodes answered with a 200.
    responses: u32,
    /// Each distinct answer, most common first.
    tally: Vec<Tally>,
}

// #region flexible_request
#[ic_cdk::update]
async fn fetch_server_time() -> Result<TimeReport, String> {
    // A committee of five, rather than every node on the subnet. Fewer requests
    // cost less and are gentler on the server's rate limits. The tradeoff is
    // that a smaller committee is easier for a single node to skew.
    let total_requests = subnet_self_node_count().min(5);
    // Return as soon as a strict majority of the committee has answered.
    let min_responses = total_requests / 2 + 1;

    // No transform function. A flexible outcall hands back each node's own
    // response instead of one the subnet agreed on, so there is nothing to make
    // identical across nodes. Leaving it off also means no cycles are reserved
    // for running it.
    let result = FlexibleHttpRequest::new("https://postman-echo.com/time/now")
        .with_max_response_bytes(1_000)
        .with_replication(ReplicationCounts {
            total_requests,
            min_responses,
            max_responses: total_requests,
        })
        .with_expected_roundtrip_time_ms(10_000)
        .send()
        .await
        .map_err(|err| format!("Outcall failed: {err}"))?;

    match result {
        // Any count between min_responses and max_responses is a normal success,
        // not a degraded one.
        FlexibleHttpRequestResult::Ok(responses) => Ok(reconcile(&responses)),
        // global_error says why the replication could not be met: a timeout,
        // out_of_cycles, responses_too_large, or too_many_rejects. node_details
        // reports what the individual nodes did.
        FlexibleHttpRequestResult::Err(err) => Err(format!(
            "Fewer than {min_responses} responses: {:?}, {}",
            err.global_error, err.message
        )),
    }
}
// #endregion flexible_request

// #region reconcile
// Reconciling the responses is the canister's job. They do not say which node
// produced them and their order is not specified, so treat them as an unordered
// multiset. Each one carries its own status, because no node had to agree with
// any other. This tallies the distinct bodies and puts the most common first.
fn reconcile(responses: &[HttpRequestResult]) -> TimeReport {
    let mut tally: Vec<Tally> = Vec::new();
    let mut counted = 0;

    for response in responses.iter().filter(|r| r.status == Nat::from(200u32)) {
        counted += 1;
        let value = String::from_utf8_lossy(&response.body).to_string();
        match tally.iter_mut().find(|entry| entry.value == value) {
            Some(entry) => entry.count += 1,
            None => tally.push(Tally { value, count: 1 }),
        }
    }

    tally.sort_by(|a, b| b.count.cmp(&a.count));
    TimeReport {
        responses: counted,
        tally,
    }
}
// #endregion reconcile

ic_cdk::export_candid!();
