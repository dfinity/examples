# Basic React Frontend

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/react)

## Overview

A minimal React application hosted entirely onchain on ICP. It demonstrates how to deploy a static frontend as an asset canister — no backend needed.

## Project structure

The `/frontend` folder contains the web assets for the application's user interface, built with React, Vite, and Tailwind CSS. The frontend is deployed as an asset canister — no backend canister is needed.

<!-- ## Deploying from ICP Ninja

This example can be deployed directly from [ICP Ninja](https://icp.ninja), a browser-based IDE for ICP.

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/hosting/react) -->

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

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
