# Who am I?

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/who_am_i)

## Overview

Who am I? demonstrates how entities on the Internet Computer are identified. Every entity, such as a user or canister smart contract, has a principal identifier. Principals can be used for identification and authentication. Who am I? uses Internet Identity (II) for user authentication, then displays the principal identifier associated with that Internet Identity on the user interface.

## Try in browser

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/dfinity/examples?devcontainer_path=.devcontainer%2Frust-who-am-i%2Fdevcontainer.json&ref=feat%2Fcodespaces)

Opens a pre-configured environment with the ICP toolchain installed. The local network starts and canisters are deployed automatically. You can browse all your Codespaces at [github.com/codespaces](https://github.com/codespaces).

> **Note:** Authentication uses production [Internet Identity](https://id.ai) rather than a local test instance. You will see your real principal identifier.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/who_am_i
```

### Deploy

Start the local network and deploy:

```bash
icp network start -d
icp deploy
```

The frontend is served by the asset canister. To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

## Ready to deploy on mainnet?

Codespaces is ideal for learning and local experimentation. When you're ready for mainnet, [install icp-cli locally](https://cli.internetcomputer.org) and follow the [mainnet deployment guide](https://cli.internetcomputer.org/0.2/guides/deploying-to-mainnet.md). Mainnet requires ICP tokens and cycles — managing identities securely is much better from your own machine.

## Updating the Candid interface

The `src/backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, rebuild the canister and regenerate the `.did` file:

```bash
icp build backend
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > src/backend/backend.did
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
