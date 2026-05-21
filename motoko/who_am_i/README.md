# Who am I?

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/who_am_i)

## Overview

Who am I? demonstrates how entities on the Internet Computer are identified. Every entity, such as a user or canister smart contract, has a principal identifier. Principals can be used for identification and authentication. Who am I? uses Internet Identity (II) for user authentication, then displays the principal identifier associated with that Internet Identity on the user interface.

## Try in browser

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/dfinity/examples?devcontainer_path=.devcontainer%2Fmotoko-who-am-i%2Fdevcontainer.json&ref=feat%2Fcodespaces)

Opens a pre-configured environment with the ICP toolchain installed and the local network started automatically. If you already have a Codespace for this example, the creation page will show an **"Open existing codespace"** option at the top — use that to resume. You can also browse all your Codespaces at [github.com/codespaces](https://github.com/codespaces).

> **Note:** Authentication uses production [Internet Identity](https://id.ai) rather than a local test instance. You will see your real principal identifier.

## Codespace actions

The local ICP network is started and canisters are deployed automatically when this Codespace opens. The frontend URL opens in your browser once deployment completes.

**Start dev server** *(optional — for frontend development)*
```sh { name=frontend }
npm run dev
```

**Reset & redeploy** *(wipes all canister state)*
```sh { name=reset-deploy }
icp deploy --mode reinstall -y
```

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
mops install
icp deploy
npm run dev
```

## Ready to deploy on mainnet?

Codespaces is ideal for learning and local experimentation. When you're ready for mainnet, [install icp-cli locally](https://cli.internetcomputer.org) and follow the [mainnet deployment guide](https://cli.internetcomputer.org/0.2/guides/deploying-to-mainnet.md). Mainnet requires ICP tokens and cycles — managing identities securely is much better from your own machine.

## Updating the Candid interface

The `src/internet_identity_app_backend/internet_identity_app_backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file:

```bash
$(mops toolchain bin moc) --idl $(mops sources) -o src/internet_identity_app_backend/internet_identity_app_backend.did src/internet_identity_app_backend/main.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
