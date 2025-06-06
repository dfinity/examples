# Rust Backend (Wasm64)

This backend-only project demonstrates how to write a simple smart contract for ICP using **Wasm64** compilation target.

This example specifically showcases building and deploying canisters compiled to the Wasm64 target, which provides access to 64-bit memory addressing and can handle larger memory spaces compared to the traditional Wasm32 target.

This application's logic is written in [Rust](https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/intro-to-rust), a programming language that can be used develop canisters on ICP.

## What is Wasm64?

The Wasm64 target allows canisters to:
- Access larger memory spaces (up to 6GiB, compared to the 4GiB limit of wasm32)
- Use 64-bit memory addressing

This example always builds for Wasm64 using Rust's nightly toolchain and the `-Z build-std` feature.

## Project structure

The `/backend` folder contains the Rust smart contract:

- `Cargo.toml`, which defines the crate that will form the backend
- `lib.rs`, which contains the actual smart contract, and exports its interface
- The `build.sh` script automatically builds for wasm64 using the nightly toolchain

## Build and deploy from the command-line

To build and deploy this wasm64 example locally, follow the instructions in the `BUILD.md` file.

The build process will automatically:
1. Install the Rust nightly toolchain if needed
2. Add the `rust-src` component for `build-std`
3. Build the canister for the `wasm64-unknown-unknown` target
4. Extract the Candid interface
5. Prepare the wasm file for deployment

