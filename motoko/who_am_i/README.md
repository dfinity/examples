# Who am I?

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/who_am_i)

## Overview

Who am I? demonstrates how entities on the Internet Computer are identified. Every entity, such as a user or canister smart contract, has a principal identifier. Principals can be used for identification and authentication. Who am I? uses Internet Identity (II) for user authentication, then displays the principal identifier associated with that Internet Identity on the user interface.

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

When done, stop the local network to free the port and clear state:

```bash
icp network stop
```

## Ready to deploy on mainnet?

When you're ready for mainnet, [install icp-cli locally](https://cli.internetcomputer.org) and follow the [mainnet deployment guide](https://cli.internetcomputer.org/0.2/guides/deploying-to-mainnet.md). Mainnet requires ICP tokens and cycles — managing identities securely is much better from your own machine.

## Updating the Candid interface

The `src/backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file using the Motoko compiler:

```bash
$(mops toolchain bin moc) --idl -o src/backend/backend.did src/backend/main.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
