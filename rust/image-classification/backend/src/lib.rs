use candid::{CandidType, Deserialize};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};
use std::cell::RefCell;

mod onnx;

// WASI polyfill requires a virtual stable memory to store the file system.
// You can replace `0` with any index up to `254`.
const WASI_MEMORY_ID: MemoryId = MemoryId::new(0);

thread_local! {
    // The memory manager is used for simulating multiple memories.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

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

#[ic_cdk::query]
fn classify(image: Vec<u8>) -> ClassificationResult {
    let result = match onnx::classify(image) {
        Ok(result) => ClassificationResult::Ok(result),
        Err(err) => ClassificationResult::Err(ClassificationError {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::query]
fn classify_query(image: Vec<u8>) -> ClassificationResult {
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
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
    onnx::setup().unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
    onnx::setup().unwrap();
}

const IMAGE: &'static [u8] = include_bytes!("../assets/man_on_ferrari_1975.png");

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
fn run() -> ClassificationResult {
    let result = classify(IMAGE.into());
    let instructions = ic_cdk::api::performance_counter(0);
    ic_cdk::println!("Executed instructions: {}", fmt(instructions));
    result
}
