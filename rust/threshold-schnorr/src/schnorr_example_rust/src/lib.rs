mod all_architectures;
pub use all_architectures::*;

// We need to compile this crate for integration testing. However, it fails to
// compile with linking errors on x86 targets, but we only need a few types
// added above for testing. For generating the canister code, `cargo build
// --release --target wasm32-unknown-unknown` should be used.
#[cfg(target_arch = "wasm32")]
mod wasm_only;
