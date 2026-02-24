---
keywords: [photo storage, store photos, photo app, photos, beginner]
---

# Photo storage example

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/photo-storage)

## Overview

The example shows how to store photos on ICP in an asset canister using the `AssetManager` from `@icp-sdk/canisters/assets`. The photo storage app is deployed as a frontend in an asset canister which is also used for photo upload.

## Project structure

The `/src` folder contains the React frontend application. The frontend is deployed as an asset canister.

## Prerequisites

- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [x] Install [Node.js](https://nodejs.org/en/download/)

## Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/photo-storage
```

## Deployment

Start the local network:

```bash
icp network start -d
```

Deploy the canisters:

```bash
icp deploy
```

The URL for the frontend depends on the canister ID. When deployed, the URL will look like this:

```
http://{canister_id}.localhost:8000
```

To authorize an identity to upload files, it must be authorized first:

```bash
icp canister call photo-storage authorize '(principal "535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe")'
```

> **Warning:** This example uses a hardcoded identity (defined in `src/App.js`). Before deploying to the IC mainnet, replace it with a proper authentication method such as [Internet Identity](https://docs.internetcomputer.org/building-apps/authentication/integrate-internet-identity).

Stop the local network when done:

```bash
icp network stop
```

## Example photos

The example cat stock photos are from [Pexels](https://www.pexels.com/license/).
