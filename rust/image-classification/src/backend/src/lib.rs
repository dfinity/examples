use candid::{CandidType, Deserialize};

mod onnx;

#[derive(CandidType, Deserialize)]
struct Classification {
    label: String,
    score: f32,
}

#[derive(CandidType, Deserialize)]
struct ClassificationError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum ClassificationResult {
    Ok(Vec<Classification>),
    Err(ClassificationError),
}

#[ic_cdk::update]
fn classify(image: Vec<u8>) -> ClassificationResult {
    let result = match onnx::classify(image) {
        Ok(result) => ClassificationResult::Ok(result),
        Err(err) => ClassificationResult::Err(ClassificationError {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::init]
fn init() {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);
    onnx::setup().unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    onnx::setup().unwrap();
}
