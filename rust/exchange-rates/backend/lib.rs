use candid::Principal;
use ic_cdk::call::{Call, Response};
use ic_xrc_types::{Asset, ExchangeRate, ExchangeRateError, GetExchangeRateRequest};

// 1B cycles per request as required by the XRC canister.
const CYCLES_PER_REQUEST: u128 = 1_000_000_000;

// The XRC canister ID is injected as PUBLIC_CANISTER_ID:xrc at deploy time:
//   local:  auto-injected by icp-cli after deploying the pre-built xrc_mock canister
//   ic:     set in icp.yaml to uf6dk-hyaaa-aaaaq-qaaaq-cai (production XRC on mainnet)
//
// See icp.yaml for the environment configuration.
fn xrc_principal() -> Principal {
    let id = ic_cdk::api::env_var_value("PUBLIC_CANISTER_ID:xrc");
    Principal::from_text(&id).expect("invalid PUBLIC_CANISTER_ID:xrc")
}

#[ic_cdk::update]
async fn get_exchange_rate(base: Asset, quote: Asset) -> u128 {
    let response: Response = Call::bounded_wait(xrc_principal(), "get_exchange_rate")
        .with_cycles(CYCLES_PER_REQUEST)
        .with_arg(&GetExchangeRateRequest {
            base_asset: base,
            quote_asset: quote,
            timestamp: None,
        })
        .await
        .expect("Call to XRC canister failed.");

    let exchange_rate_result: Result<ExchangeRate, ExchangeRateError> =
        response.candid().expect("Decoding result failed.");

    ic_cdk::println!("result: {:?}", exchange_rate_result);

    exchange_rate_result.unwrap().rate as u128
}

ic_cdk::export_candid!();
