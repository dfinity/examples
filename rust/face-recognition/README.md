# ICP face recognition

This is an ICP smart contract that runs face detection and face recognition on user photos that can be uploaded either from a camera or a local file.

The smart contract consists of two canisters:

- The **backend canister** embeds the [Tract ONNX inference engine](https://github.com/sonos/tract) with two ONNX models. One model is used to detect a face in the photo and return its bounding box. Another model is used for computing face embeddings.
- The **frontend canister** contains the Web assets such as HTML, JS, and CSS that are served to the browser.

## Models

The smart contract uses two models: one for detecting the face and another for recognizing the face.

Since the models are large, they cannot be embedded into the Wasm binary of the smart contract. Instead they must be uploaded separately after deployment.

### Face detection

A face detection model finds the bounding box of a face in the image.
You can download [Ultraface](https://github.com/onnx/models/tree/main/validated/vision/body_analysis/ultraface) — ultra-lightweight face detection model — by running:

```bash
./download-face-detection-model.sh
```

### Face recognition

A face recognition model computes a vector embedding of an image with a face.
You can obtain a pretrained model from [facenet-pytorch](https://github.com/timesler/facenet-pytorch) as follows.

1. Install `python` and `pip`: https://packaging.python.org/en/latest/tutorials/installing-packages/

2. Install `facenet-pytorch`, `torch`, and `onnx`:

   ```bash
   pip install facenet-pytorch
   pip install torch
   pip install onnx
   ```

3. Export the ONNX model. Start a Python shell and run:

   ```python
   import torch
   import facenet_pytorch
   resnet = facenet_pytorch.InceptionResnetV1(pretrained='vggface2').eval()
   input = torch.randn(1, 3, 160, 160)
   torch.onnx.export(resnet, input, "face-recognition.onnx", verbose=False, opset_version=11)
   ```

   This produces `face-recognition.onnx`. Copy the file to the root of this repository.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/face-recognition
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

Run `npm run dev` from the `frontend/` directory for hot reload during frontend development.

### Upload the ONNX models

After deploying, upload the ONNX models to the canister using [ic-file-uploader](https://github.com/modclub-app/ic-file-uploader/tree/main):

```bash
cargo install ic-file-uploader
make upload-models
```

Once the models are uploaded, open the frontend URL in your browser to interact with the smart contract.

## Credits

Thanks to [DecideAI](https://decideai.xyz/) for discussions and providing [ic-file-uploader](https://github.com/modclub-app/ic-file-uploader/tree/main).

## Security considerations and best practices

See the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview).
