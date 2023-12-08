use candid::{CandidType, Deserialize};
use std::include_bytes;

mod core;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO: &[u8] = include_bytes!("../assets/logo.png");

#[derive(CandidType, Deserialize)]
struct Options {
    add_logo: bool,
    add_gradient: bool,
}

#[derive(CandidType, Deserialize)]
struct QrError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Err(QrError),
}

fn qrcode_impl(input: String, options: Options) -> QrResult {
    let result = match core::generate(input, options, LOGO, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob) => QrResult::Image(blob),
        Err(err) => QrResult::Err(QrError {
            message: err.to_string(),
        }),
    };
    ic_cdk::println!(
        "Executed instructions: {}",
        ic_cdk::api::performance_counter(0)
    );
    result
}

#[ic_cdk::update]
fn qrcode(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}
