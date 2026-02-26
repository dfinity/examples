# HTTP: POST

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/send_http_post)

## Overview

The purpose of this dapp is to give developers a minimal dapp that uses the IC's HTTPS outcalls feature to make a `POST` request.

This demo goes in hand with the [developer documentation on HTTPS outcalls](https://docs.internetcomputer.org/building-apps/network-features/using-http/https-outcalls/post).

## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP. To continue developing locally after deploying from ICP Ninja, see [BUILD.md](BUILD.md).

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/send_http_post)

> **Note:** ICP Ninja currently uses `dfx` under the hood, which is why this example includes a `dfx.json` configuration file. The `Makefile` also uses `dfx` for ICP Ninja CI compatibility. `dfx` is the legacy CLI, being superseded by [icp-cli](https://cli.icp.build), which is what developers should use for local development.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/send_http_post
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
