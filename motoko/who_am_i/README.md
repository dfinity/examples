# Who am I?

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/who_am_i)

## Overview

Who am I? demonstrates how entities on the Internet Computer are identified. Every entity, such as a user or canister smart contract, has a principal identifier. Principals can be used for identification and authentication. Who am I? uses Internet Identity (II) for user authentication, then displays the principal identifier associated with that Internet Identity on the user interface.

## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP. To continue developing locally after deploying from ICP Ninja, see [BUILD.md](BUILD.md).

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/motoko/who_am_i)

> **Note:** ICP Ninja currently uses `dfx` under the hood, which is why this example includes a `dfx.json` configuration file. `dfx` is the legacy CLI, being superseded by [icp-cli](https://cli.internetcomputer.org), which is what developers should use for local development.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/who_am_i
```

### Deployment

Start the local network:

```bash
icp network start -d
```

Deploy the canisters:

```bash
icp deploy
```

Stop the local network when done:

```bash
icp network stop
```

## Updating the Candid interface

The `src/internet_identity_app_backend/internet_identity_app_backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file:

```bash
$(mops toolchain bin moc) --idl $(mops sources) -o src/internet_identity_app_backend/internet_identity_app_backend.did src/internet_identity_app_backend/main.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
