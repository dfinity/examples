use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::include_bytes;

mod core;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const QR_CODE_NUMBER_OF_ITERATIONS: usize = 10;
const LOGO_TRANSPARENT: &[u8] = include_bytes!("../assets/logo_transparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("../assets/logo_white.png");

thread_local! {
    static STATE: RefCell<Vec<QrResult>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Clone, Deserialize)]
struct Options {
    add_logo: bool,
    add_gradient: bool,
    add_transparency: Option<bool>,
}

#[derive(Clone, CandidType, Deserialize)]
struct QrError {
    message: String,
}

#[derive(Clone, CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Err(QrError),
}

fn qrcode_impl(input: String, options: Options) -> QrResult {
    let logo = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENT
    } else {
        LOGO_WHITE
    };
    let result = match core::generate(input, options, logo, IMAGE_SIZE_IN_PIXELS) {
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
    let mut res = vec![];

    for _ in 0..QR_CODE_NUMBER_OF_ITERATIONS {
        res.push(qrcode_impl(input.clone(), options.clone()));
    }

    let r = res[0].clone();

    STATE.with(|state| {
        state.borrow_mut().extend(res);
    });
    r
}

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    let mut r = None;
    for _ in 0..QR_CODE_NUMBER_OF_ITERATIONS {
        r = Some(qrcode_impl(input.clone(), options.clone()));
    }
    r.unwrap()
}

#[ic_cdk::query]
fn qrresult_len() -> u64 {
    STATE.with(|state| state.borrow().len().try_into().unwrap())
}

#[ic_cdk::query]
fn qrcode_query_single(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}
