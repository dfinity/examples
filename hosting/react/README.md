# Basic React Frontend

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/hosting/react)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

## Overview

A minimal React application hosted entirely onchain on ICP. It demonstrates how to deploy a static frontend as an asset canister — no backend needed.

## Project structure

The `/frontend` folder contains the web assets for the application's user interface, built with React, Vite, and Tailwind CSS. The frontend is deployed as an asset canister — no backend canister is needed.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/react
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
