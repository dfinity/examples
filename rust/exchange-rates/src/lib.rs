use candid::Principal;
use ic_cdk::call::{Call, Response};
use ic_xrc_types::{Asset, ExchangeRate, ExchangeRateError, GetExchangeRateRequest};

const EXCHANGE_RATE_CANISTER: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";
const CYCLES_PER_REQUEST: u128 = 1_000_000_000;

#[ic_cdk::update]
async fn get_exchange_rate(base: Asset, quote: Asset) -> u128 {
    let exchange_rate_principal = Principal::from_text(EXCHANGE_RATE_CANISTER).unwrap();

    let response: Response = Call::bounded_wait(exchange_rate_principal, "get_exchange_rate")
        .with_cycles(CYCLES_PER_REQUEST)
        .with_arg(&GetExchangeRateRequest {
            base_asset: base,
            quote_asset: quote,
            timestamp: None,
        })
        .await
        .expect("Call to exchange canister failed.");

    let exchange_rate_result: Result<ExchangeRate, ExchangeRateError> =
        response.candid().expect("Decoding result failed.");

    ic_cdk::println!("result: {:?}", exchange_rate_result);

    exchange_rate_result.unwrap().rate as u128
}
