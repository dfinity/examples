# EVM Block Explorer

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/evm_block_explorer)

## Overview

The EVM Block Explorer example demonstrates how an ICP canister can fetch block data directly from Ethereum and other EVM-compatible chains. Using HTTPS outcalls via the [EVM RPC canister](https://github.com/dfinity/evm-rpc-canister), canisters on ICP can read on-chain data without a bridge or oracle. The same pattern applies to any EVM-compatible chain supported by the EVM RPC canister.

<!--
## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP.

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/motoko/evm_block_explorer)
-->

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [ic-mops](https://mops.one/): `npm install -g ic-mops`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/evm_block_explorer
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

On mainnet, the `evm_rpc` canister is already deployed at `7hfb6-caaaa-aaaar-qadga-cai`. Deploy only the backend and frontend — icp-cli injects the correct canister ID via the `ic` environment configuration in `icp.yaml`:

```bash
icp deploy -e ic
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file:

```bash
mops generate candid backend
```

## RPC providers and API keys

The example uses [PublicNode](https://ethereum-rpc.publicnode.com) by default — a free, no-registration provider that works out of the box locally and on mainnet. This is sufficient for getting started and automated testing.

For production deployments requiring premium providers (Alchemy, Ankr, BlockPi), refer to the [EVM RPC canister documentation](https://github.com/dfinity/evm-rpc-canister) for how to configure API keys. Once configured, change `#EthMainnet(?[#PublicNode])` in `backend/EvmRpcApi.mo` to `#EthMainnet(null)` to use all configured providers for better consensus.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
