# ICP face recognition

This example demonstrates running face detection and face recognition inside an ICP canister using the [Tract ONNX inference engine](https://github.com/sonos/tract). Users can upload photos from a camera or local file, detect faces, and identify people by name.

The example consists of two canisters:

- **backend** — embeds the Tract ONNX inference engine. Exposes endpoints for uploading ONNX model files in chunks, loading them into memory, detecting faces, computing face embeddings, and recognizing people. Also exposes `run_detection` and `run_recognition` which run the models against a built-in test image and log the IC instruction count — useful for smoke-testing and capacity planning:

  ```bash
  icp canister call --query backend run_detection '()'
  icp canister call backend run_recognition '()'
  icp canister logs backend   # shows the instruction count logged by each call
  ```
- **frontend** — serves the web UI (HTML/JS/CSS).

## Models

The backend uses two ONNX models that are too large to embed in the Wasm binary and must be uploaded after deployment. `icp deploy` handles this automatically via its sync phase:

- **Face detection** ([Ultraface](https://github.com/onnx/models/tree/main/validated/vision/body_analysis/ultraface)) — downloaded automatically.
- **Face recognition** ([facenet-pytorch](https://github.com/timesler/facenet-pytorch) InceptionResnetV1) — generated automatically if Python 3.9–3.12 is available (`facenet-pytorch`, `torch`, and `onnx` are installed via pip). If no compatible Python is found, place `face-recognition.onnx` in the project root manually and run `icp deploy` again.

Models are stored in stable memory and survive canister upgrades — they reload automatically without re-uploading. Note: the persons database (added via the frontend) is stored in heap memory and is cleared on upgrade.

## Build and deploy from the command line

### Prerequisites

**Required:**

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- [wasi2ic](https://github.com/wasm-forge/wasi2ic): `cargo install wasi2ic`

`wasm-opt` is installed automatically on first deploy if not already present.

**Optional (for automatic face recognition model generation):**

- Python 3.9–3.12 with pip — the sync phase auto-installs `facenet-pytorch`, `torch`, and `onnx` and generates `face-recognition.onnx`. Python 3.13+ is not yet supported by torch.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/face-recognition
```

### Deploy

```bash
icp network start -d
icp deploy
icp network stop
```

`icp deploy` runs three phases:
1. **Build** — compiles the Rust backend to WASM (via wasm32-wasip1 + wasi2ic).
2. **Deploy** — installs the backend and frontend canisters.
3. **Sync** — downloads the face detection model, generates the face recognition model (if Python 3.9–3.12 is available), and uploads both to the canister. Skipped on redeployment if models are already loaded.

After deployment the CLI prints the frontend URL. Open it in a browser to interact with the canister.

### Test

```bash
bash test.sh
```

`test.sh` exercises the model management API without requiring models to be loaded. The frontend shows a setup instruction if models are not yet uploaded.

For frontend development with hot reload:

```bash
npm run dev --prefix frontend
```

## Updating the Candid interface

Only needed if you change the backend endpoints. Requires `candid-extractor` (`cargo install candid-extractor`):

```bash
icp build backend && candid-extractor ./target/wasm32-wasip1/release/backend.wasm > backend/backend.did
```

## Credits

Thanks to [DecideAI](https://decideai.xyz/) for discussions and providing [ic-file-uploader](https://github.com/decide-ai/ic-file-uploader).

## Security considerations and best practices

See the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview).
