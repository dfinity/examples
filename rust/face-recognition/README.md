# ICP face recognition

This is an ICP smart contract that runs face detection and face recognition on user photos that can be uploaded either from a camera or a local file.

The smart contract consists of two canisters:

- The **backend canister** embeds the [Tract ONNX inference engine](https://github.com/sonos/tract) with two ONNX models. One model is used to detect a face in the photo and return its bounding box. Another model is used for computing face embeddings.
- The **frontend canister** contains the Web assets such as HTML, JS, and CSS that are served to the browser.

## Models

The smart contract uses two models: one for detecting faces and another for recognizing them.

Since the models are large they cannot be embedded in the Wasm binary — they are uploaded to the canister after deployment. The `icp deploy` sync phase handles this automatically:

- **Face detection** ([Ultraface](https://github.com/onnx/models/tree/main/validated/vision/body_analysis/ultraface)) — downloaded automatically.
- **Face recognition** ([facenet-pytorch](https://github.com/timesler/facenet-pytorch) InceptionResnetV1) — generated automatically if `python3` is available (installs `facenet-pytorch`, `torch`, and `onnx` via pip). If `python3` is not available, place `face-recognition.onnx` in the project root manually and run `icp deploy` again.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- [wasi2ic](https://github.com/wasm-forge/wasi2ic): `cargo install wasi2ic`

`wasm-opt` is installed automatically on first deploy if not already present.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/face-recognition
```

### Deploy and test

If the ONNX model files are present before running `icp deploy`, they are uploaded automatically to the canister as part of the sync phase. `icp deploy` skips the upload on redeployment if the models are already loaded.

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`bash test.sh` exercises the model management API. The frontend requires the models to be loaded — open the URL printed by `icp deploy` in your browser. If the models are not yet uploaded, the frontend shows a setup instruction.

For frontend development with hot reload:

```bash
npm run dev --prefix frontend
```

## Updating the Candid interface

Only needed if you change the backend endpoints. Requires `candid-extractor` (`cargo install candid-extractor`) and `ic_cdk::export_candid!()` at the end of `backend/src/lib.rs` (already present):

```bash
icp build backend && candid-extractor ./target/wasm32-wasip1/release/backend.wasm > backend/backend.did
```

## Credits

Thanks to [DecideAI](https://decideai.xyz/) for discussions and providing [ic-file-uploader](https://github.com/decide-ai/ic-file-uploader).

## Security considerations and best practices

See the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview).
