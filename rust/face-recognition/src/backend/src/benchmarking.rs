// The code below is used for testing and benchmarking.

use crate::{onnx, Detection, Error, Recognition};

const IMAGE: &'static [u8] = include_bytes!("../assets/image.png");

/// Formats thousands for the specified `u64` integer (helper function).
fn fmt(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

#[ic_cdk::query]
fn run_detection() -> Detection {
    let result = match onnx::detect(IMAGE.to_vec()) {
        Ok(result) => Detection::Ok(result.0),
        Err(err) => Detection::Err(Error {
            message: err.to_string(),
        }),
    };
    let instructions = ic_cdk::api::performance_counter(0);
    ic_cdk::println!("Executed instructions: {}", fmt(instructions));
    result
}

#[ic_cdk::update]
fn run_recognition() -> Recognition {
    let result = match onnx::recognize(IMAGE.to_vec()) {
        Ok(result) => Recognition::Ok(result),
        Err(err) => Recognition::Err(Error {
            message: err.to_string(),
        }),
    };
    let instructions = ic_cdk::api::performance_counter(0);
    ic_cdk::println!("Executed instructions: {}", fmt(instructions));
    result
}
