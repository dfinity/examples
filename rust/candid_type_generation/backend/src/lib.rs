// The `declarations` module includes Rust types generated at build time from
// candid/nns_governance.did via build.rs. See src/declarations/mod.rs.
mod declarations;

use declarations::nns_governance::{ListNeurons, ListNeuronsResponse};

/// Fetches neurons from the NNS Governance canister and returns them as a JSON string.
///
/// This must be an update call (not a query) because it makes an inter-canister call.
/// Inter-canister calls are only allowed from update calls on the IC.
#[ic_cdk::update]
async fn list_neurons_pretty() -> String {
    match fetch_neurons().await {
        Ok(response) => match serde_json::to_string_pretty(&response) {
            Ok(json) => format!("NNS Governance Neurons:\n{}", json),
            Err(_) => format!("NNS Governance Neurons (Debug):\n{:#?}", response),
        },
        Err(err) => format!("Error fetching neurons: {}", err),
    }
}

/// Makes a typed inter-canister call to the NNS Governance canister.
///
/// `declarations::nns_governance::list_neurons` is a generated function that:
///   1. Encodes `arg` as Candid
///   2. Calls the `list_neurons` method on the NNS Governance canister
///   3. Decodes the response as `ListNeuronsResponse`
///
/// The canister ID (`rrkah-fqaaa-aaaaa-aaaaq-cai`) is embedded as a constant in the
/// generated code — set in build.rs via `.static_callee(principal)`.
async fn fetch_neurons() -> Result<ListNeuronsResponse, String> {
    let request = ListNeurons {
        neuron_ids: vec![],
        include_neurons_readable_by_caller: true,
        include_empty_neurons_readable_by_caller: Some(false),
        include_public_neurons_in_full_neurons: Some(true),
        page_number: None,
        page_size: Some(10),
        neuron_subaccounts: None,
    };

    declarations::nns_governance::list_neurons(&request)
        .await
        .map_err(|e| format!("Inter-canister call failed: {}", e))
}

ic_cdk::export_candid!();
