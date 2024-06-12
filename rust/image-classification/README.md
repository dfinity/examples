# ICP Image Classification

This is an ICP smart contract that accepts an image from the user and runs image classification inference.
The smart contract consists of two canisters:

- the backend canister embeds the [the Tract ONNX inference engine](https://github.com/sonos/tract) with [the MobileNet v2-7 model](https://github.com/onnx/models/tree/main/validated/vision/classification/mobilenet).
  It provides `classify()` and `classify_query()` endpoints for the frontend code to call.
  The former endpoint is used for replicated execution (running on all nodes) whereas the latter runs only on a single node.
- the frontend canister contains the Web assets such as HTML, JS, CSS that are served to the browser.

This example uses Wasm SIMD instructions that are available in `dfx` version `0.20.2-beta.0` or newer.

# Dependencies

Install `dfx`, Rust, etc: https://internetcomputer.org/docs/current/developer-docs/getting-started/hello-world

Install WASI SDK 21:

- Install `wasi-skd-21.0` from https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-21
- Export `CC_wasm32_wasi` in your shell such that it points to WASI clang and sysroot. Example:

```
export CC_wasm32_wasi="/path/to/wasi-sdk-21.0/bin/clang --sysroot=/path/to/wasi-sdk-21.0/share/wasi-sysroot"
``` 

Install `wasi2ic`:
- Follow the steps in https://github.com/wasm-forge/wasi2ic
- Make sure that `wasi2ic` binary is in your `$PATH`.

Download MobileNet v2-7 to `src/backend/assets/mobilenetv2-7.onnx`:

```
./downdload_model.sh
```

Install NodeJS dependencies for the frontend:

```
npm install
```

Install `wasm-opt`:

```
cargo install wasm-opt
```

# Build

```
dfx start --background
dfx deploy
```

If the deployment is successful, the it will show the `frontend` URL.
Open that URL in browser to interact with the smart contract.
