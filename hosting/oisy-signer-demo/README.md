# OISY Signer Demo

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/hosting/oisy-signer-demo)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

## Overview

A sample application demonstrating interaction with the [OISY Wallet](https://oisy.com). It connects to the OISY signer, fetches balances for **TESTICP** and **TICRC1** (testnet tokens), and performs self-transfers of 1 token each using the ICRC-1 standard. No backend canister is needed — the frontend is deployed as an asset canister.

Testnet tokens can be obtained for free using the [ICP Faucet](https://faucet.internetcomputer.org). In OISY, select the **IC (testnet tokens)** network to view them.

## Project structure

The `/frontend` folder contains the web assets for the application's user interface, built with React, Vite, and Tailwind CSS.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/oisy-signer-demo
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

`icp deploy` prints the frontend URL. On the local network it is derived from the canister name:

```
http://frontend.local.localhost:8000
```

Stop the local network when done:

```bash
icp network stop
```
