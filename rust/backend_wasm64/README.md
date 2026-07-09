# Rust backend (Wasm64)

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/backend_wasm64)

## Overview

This backend-only project demonstrates how to write a simple smart contract for ICP using **Wasm64** compilation target.

This example specifically showcases building and deploying canisters compiled to the Wasm64 target, which provides access to 64-bit memory addressing and can handle larger memory spaces compared to the traditional Wasm32 target.

## What is Wasm64?

The Wasm64 target allows canisters to:

- Access larger memory spaces (up to 6GiB, compared to the 4GiB limit of Wasm32)
- Use 64-bit memory addressing

This example uses the `build.sh` script to build for Wasm64 using Rust's nightly toolchain and the `-Z build-std` feature.

<!--
[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/backend_wasm64)
-->

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/backend_wasm64
```

### Deploy and test

Start the local network:

```bash
icp network start -d
```

Deploy the canister:

```bash
icp deploy
```

Run the tests:

```bash
bash test.sh
```

Stop the local network when done:

```bash
icp network stop
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
