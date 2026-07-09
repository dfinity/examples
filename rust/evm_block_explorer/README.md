# EVM Block Explorer

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/evm_block_explorer)

## Overview

The EVM Block Explorer example demonstrates how an ICP smart contract can obtain information directly from other blockchain networks. Using HTTPS outcalls, smart contracts on ICP can interact with other networks without needing to go through a third-party service such as a bridge or an oracle. Supported interactions with other chains include querying network data, signing transactions, and submitting transactions directly to other networks.
In this example, you'll also see how to sign transactions with canister ECDSA or Schnorr signatures.

<!--
## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP.

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/evm_block_explorer)
-->

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/evm_block_explorer
```

### Deploy and test

Start the local network, deploy all canisters (including the local EVM RPC canister), and run the tests:

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

To start a frontend dev server with hot reload during frontend development:

```bash
npm run dev --prefix frontend
```

### Deploying on ICP mainnet

On mainnet, the `evm_rpc` canister is already deployed at `7hfb6-caaaa-aaaar-qadga-cai`. Deploy only the backend and frontend — icp-cli injects the correct canister ID via the `production` environment configuration in `icp.yaml`:

```bash
icp deploy backend frontend -e production
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, rebuild the canister and regenerate the `.did` file:

```bash
icp build backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
