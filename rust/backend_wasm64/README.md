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

## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP. To continue developing locally after deploying from ICP Ninja, see [BUILD.md](BUILD.md).

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/backend_wasm64)

> **Note:** ICP Ninja currently uses `dfx` under the hood, which is why this example includes a `dfx.json` configuration file. `dfx` is the legacy CLI, being superseded by [icp-cli](https://cli.icp.build), which is what developers should use for local development.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/backend_wasm64
```

### Deployment

Start the local network:

```bash
icp network start -d
```

Deploy the canister:

```bash
icp deploy
```

Stop the local network when done:

```bash
icp network stop
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
