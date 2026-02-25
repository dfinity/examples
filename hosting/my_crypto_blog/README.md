# My Crypto Blog

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/my_crypto_blog)

## Overview

A simple blog-style web application hosted entirely onchain on ICP. Built with React, Vite, and Tailwind CSS, it demonstrates how to deploy a frontend-only application as an asset canister — no backend needed.

The app fetches blog posts from an external API and displays them in a card layout.

## Project structure

The `/frontend` folder contains the web assets for the application's user interface, built with React, Vite, and Tailwind CSS. The frontend is deployed as an asset canister — no backend canister is needed.

## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP. To continue developing locally after deploying from ICP Ninja, see [BUILD.md](BUILD.md).

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/hosting/my_crypto_blog)

> **Note:** ICP Ninja currently uses `dfx` under the hood, which is why this example includes a `dfx.json` configuration file. `dfx` is the legacy CLI, being superseded by [icp-cli](https://cli.icp.build), which is what developers should use for local development.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/my_crypto_blog
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

The URL for the frontend depends on the canister ID. When deployed, the URL will look like this:

```
http://{canister_id}.localhost:8000
```

Stop the local network when done:

```bash
icp network stop
```
