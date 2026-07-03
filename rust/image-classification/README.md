# ICP image classification

This example demonstrates running an ONNX machine-learning model inside an ICP canister.
The smart contract accepts an image from the user and runs image classification inference using the [Tract ONNX inference engine](https://github.com/sonos/tract) with the [MobileNet v2-7 model](https://github.com/onnx/models/tree/main/validated/vision/classification/mobilenet).

The example uses the WASI polyfill to run Tract (which relies on POSIX file I/O) inside the deterministic ICP runtime, and Wasm SIMD instructions for faster inference.

The smart contract consists of two canisters:

- **backend** — embeds the Tract ONNX inference engine with the MobileNet v2-7 model.
  It provides `classify()` and `classify_query()` endpoints:
  the former runs under replicated execution (all nodes), the latter runs on a single node as a query call.
- **frontend** — serves the web UI (HTML/JS/CSS) from which users upload images and view results.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- [wasi2ic](https://github.com/wasm-forge/wasi2ic): ensure `wasi2ic` is in your `$PATH`
- [wasm-opt](https://github.com/WebAssembly/binaryen): `cargo install wasm-opt`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/image-classification
```

Download the MobileNet v2-7 model:

```bash
./download_model.sh
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

If the deployment is successful, the CLI will print the frontend URL.
Open that URL in a browser to interact with the smart contract.

For frontend development with hot reload:

```bash
npm run dev --prefix frontend
```

## Updating the Candid interface

```bash
icp build backend && candid-extractor target/wasm32-wasip1/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

Refer to the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview) for guidance on securing your canister.
