# Rust backend (Wasm64)

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/rust/backend_wasm64)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

## Overview

This backend-only project demonstrates how to write a simple canister for ICP using **Wasm64** compilation target.

This example specifically showcases building and deploying canisters compiled to the Wasm64 target, which provides access to 64-bit memory addressing and can handle larger memory spaces compared to the traditional Wasm32 target.

## What is Wasm64?

The Wasm64 target allows canisters to:

- Access larger memory spaces (up to 6GiB, compared to the 4GiB limit of Wasm32)
- Use 64-bit memory addressing

This example uses the `build.sh` script to build for Wasm64 using Rust's nightly toolchain and the `-Z build-std` feature.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with nightly toolchain (installed automatically by `build.sh`)

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
