use candid::{CandidType, Deserialize};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl,
};
use onnx::{BoundingBox, Embedding, Person};
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
struct Error {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum Detection {
    Ok(BoundingBox),
    Err(Error),
}

#[derive(CandidType, Deserialize)]
enum Addition {
    Ok(Embedding),
    Err(Error),
}

#[derive(CandidType, Deserialize)]
enum Recognition {
    Ok(Person),
    Err(Error),
}

#[ic_cdk::query]
fn detect(image: Vec<u8>) -> Detection {
    let result = match onnx::detect(image) {
        Ok(result) => Detection::Ok(result.0),
        Err(err) => Detection::Err(Error {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::query]
fn recognize(image: Vec<u8>) -> Recognition {
    let result = match onnx::recognize(image) {
        Ok(result) => Recognition::Ok(result),
        Err(err) => Recognition::Err(Error {
            message: err.to_string(),
        }),
    };
    result
}

#[ic_cdk::update]
fn add(label: String, image: Vec<u8>) -> Addition {
    let result = match onnx::add(label, image) {
        Ok(result) => Addition::Ok(result),
        Err(err) => Addition::Err(Error {
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
