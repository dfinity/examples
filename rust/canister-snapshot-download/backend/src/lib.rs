use ic_cdk::{query, update};
use ic_stable_structures::{DefaultMemoryImpl, Memory};

#[update]
fn setup() {
    let mem = DefaultMemoryImpl::default();
    assert_ne!(mem.grow(1), -1, "failed to grow stable memory");
    mem.write(0, b"Colourless green ideas sleep furiously.");
}

#[query]
fn print() -> String {
    let mem = DefaultMemoryImpl::default();
    let mut buf = vec![0u8; 39];
    mem.read(0, &mut buf);
    String::from_utf8(buf).unwrap()
}

ic_cdk::export_candid!();
