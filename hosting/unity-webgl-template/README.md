---
keywords: [unity, unity webgl, beginner]
---

# Unity WebGL sample

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/unity-webgl-template)

## Overview

The example shows how to deploy a simple Unity WebGL build on ICP in an asset canister. It just shows a Unity WebGL build with the URP template installed. The Unity WebGL build is deployed as frontend, no backend is needed in this sample.

## Project structure

The `/frontend` folder contains the pre-built Unity WebGL export. The frontend is deployed as an asset canister.

## Prerequisites

- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

## Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/unity-webgl-template
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

Stop the local network when done:

```bash
icp network stop
```
