mod declarations;

use declarations::nns_governance::{ListNeurons, ListNeuronsResponse};

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
