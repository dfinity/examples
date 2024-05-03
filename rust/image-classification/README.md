# ICP Image Classification

This is an ICP smart contract that accepts an image from the user and runs image classification inference.
The smart contract consists of two canisters:

- the backend canister embeds the [the Tract ONNX inference engine](https://github.com/sonos/tract) with [the MobileNet v2-7 model](https://github.com/onnx/models/tree/main/validated/vision/classification/mobilenet). It provides a `classify()` endpoint for the frontend code to call.
- the frontend canister contains the Web assets such as HTML, JS, CSS that are served to the browser.

Note that currently Wasm execution is not optimized for this workload.
A single call executes about 24B instructions (~10s).

This is expected to improve in the future with:

- faster deterministic floating-point operations.
- Wasm SIMD (Single-Instruction Multiple Data).

The ICP mainnet subnets and `dfx` running a replica version older than [463296](https://dashboard.internetcomputer.org/release/463296c0bc82ad5999b70245e5f125c14ba7d090) may fail with an instruction-limit-exceeded error.

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

# Build

```
dfx start --background
dfx deploy
```

If the deployment is successfull, the it will show the `frontend` URL.
Open that URL in browser to interact with the smart contract.
