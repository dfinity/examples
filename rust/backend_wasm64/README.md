# Rust backend (Wasm64)

This backend-only project demonstrates how to write a simple smart contract for ICP using **Wasm64** compilation target.

This example specifically showcases building and deploying canisters compiled to the Wasm64 target, which provides access to 64-bit memory addressing and can handle larger memory spaces compared to the traditional Wasm32 target.

## What is Wasm64?

The Wasm64 target allows canisters to:
- Access larger memory spaces (up to 6GiB, compared to the 4GiB limit of Wasm32)
- Use 64-bit memory addressing

This example uses the `build.sh` script to build for Wasm64 using Rust's nightly toolchain and the `-Z build-std` feature. 

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/backend_wasm64)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.

