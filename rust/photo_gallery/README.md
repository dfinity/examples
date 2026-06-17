# Photo Gallery

A decentralized photo gallery application built on the Internet Computer. Users can upload and view photos stored directly on-chain, served via the HTTP gateway with browser-cacheable responses.

**Note:** This example is primarily a demo showing how the [response verification library](https://docs.internetcomputer.org/references/http-gateway-protocol-spec) and HTTP gateways can serve images with long-lived `Cache-Control` headers. It does not implement authentication or per-user access control. Use it at your own risk.

## Overview

The backend canister stores images in memory and exposes three methods:

- `upload_image(name, content_type, data)` — stores an image blob and returns its numeric ID.
- `list_images()` — returns metadata (ID, name, content type) for all stored images.
- `http_request(request)` — serves images at `/image/<id>` via the HTTP gateway, including skip-certification headers and `Cache-Control: public, max-age=31536000, immutable` so browsers cache images after the first fetch.

The frontend is a React + Vite application that uploads images and renders the gallery by constructing HTTP gateway URLs for each image ID, allowing the browser to fetch and cache image data directly.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/en/download/)
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/photo_gallery
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

## Updating the Candid interface

If you modify the backend's public API, rebuild the canister and regenerate the `.did` file:

```bash
icp build backend
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
