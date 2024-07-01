use candid::{CandidType, Deserialize};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};
use onnx::{BoundingBox, Embedding};
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
struct DetectionError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum DetectionResult {
    Ok(BoundingBox),
    Err(DetectionError),
}

#[derive(CandidType, Deserialize)]
struct EmbeddingError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum EmbeddingResult {
    Ok(Embedding),
    Err(EmbeddingError),
}


#[ic_cdk::update]
fn detect(image: Vec<u8>) -> DetectionResult {
    let result = match onnx::detect(image) {
        Ok(result) => DetectionResult::Ok(result.0),
        Err(err) => DetectionResult::Err(DetectionError {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::query]
fn detect_query(image: Vec<u8>) -> DetectionResult {
    ic_cdk::api::print("query started");
    let result = match onnx::detect(image) {
        Ok(result) => DetectionResult::Ok(result.0),
        Err(err) => DetectionResult::Err(DetectionError {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::update]
fn embedding(image: Vec<u8>) -> EmbeddingResult {
    let result = match onnx::embedding(image) {
        Ok(result) => EmbeddingResult::Ok(result),
        Err(err) => EmbeddingResult::Err(EmbeddingError {
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
