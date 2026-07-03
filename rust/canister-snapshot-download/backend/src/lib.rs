use ic_cdk::{query, update};
use ic_stable_structures::{DefaultMemoryImpl, Memory};

// The canister stores a single text quote in the first stable memory page.
// This lets the snapshot example demonstrate a meaningful state change:
// the quote is written with a British spelling ("Colourless"), then fixed
// offline to American spelling ("Colorless") via snapshot manipulation.

/// Write the initial quote into stable memory page 0.
#[update]
fn setup() {
    let mem = DefaultMemoryImpl::default();
    // grow returns -1 on failure (out of memory).
    assert_ne!(mem.grow(1), -1, "failed to grow stable memory");
    mem.write(0, b"Colourless green ideas sleep furiously.");
}

/// Read the quote back from stable memory.
#[query]
fn print() -> String {
    let mem = DefaultMemoryImpl::default();
    let mut buf = vec![0u8; 39];
    mem.read(0, &mut buf);
    String::from_utf8(buf).unwrap()
}

ic_cdk::export_candid!();
