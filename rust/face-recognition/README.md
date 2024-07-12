# ICP Face Recognition

This is an ICP smart contract runs face detection and face recognition of user's photo that can be uploaded either from a camera or a local file.

The smart contract consists of two canisters:

- the backend canister embeds the [the Tract ONNX inference engine](https://github.com/sonos/tract) with two ONNX models. One model is used to detect a face in the photo and return its bounding box. Another model is used for computing face embeddings.
- the frontend canister contains the Web assets such as HTML, JS, CSS that are served to the browser.

# Models

The smart contract uses two models: one for detecting the face and another for recognizing the face.

## Face detection

A face detection model finds the bounding box of a face in the image.
You can download [Ultraface](https://github.com/onnx/models/tree/main/validated/vision/body_analysis/ultraface) - ultra-lightweight face detection model - [[here](https://github.com/onnx/models/blob/bec48b6a70e5e9042c0badbaafefe4454e072d08/validated/vision/body_analysis/ultraface/models/version-RFB-320.onnx)].

Alternatively, you can run
```
./download-face-detection-model.sh
```

## Face recognition

A face recognition model computes a vector embedding of an image with a face.
You can obtain a pretrained model from [facenet-pytorch](https://github.com/timesler/facenet-pytorch) as follows.


1. Install `python` and `pip`: https://packaging.python.org/en/latest/tutorials/installing-packages/.

2. Install `facenet-pytorch` and  `torch`:
```
pip install facenet-pytorch
pip install torch
pip install onnx
```

3. Export ONNX model. Start a python shell and run the following commands or create a python file and run it:
```
import torch
import facenet_pytorch
resnet = facenet_pytorch.InceptionResnetV1(pretrained='vggface2').eval()
input = torch.randn(1, 3, 160, 160)
torch.onnx.export(resnet, input, "face-recognition.onnx", verbose=False, opset_version=11)
```

4. This should produce `face-recognition.onnx`. Copy the file to the root of this repository.

# Dependencies

Install `dfx`, Rust, etc: https://internetcomputer.org/docs/current/developer-docs/getting-started/hello-world

Install `wasi2ic`:
- Follow the steps in https://github.com/wasm-forge/wasi2ic
- Make sure that `wasi2ic` binary is in your `$PATH`.

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

# Chunk uploading of models

Since the models are large, they cannot be embedded into the Wasm binary of the smart contract.
Instead they should be uploaded separately.

[DecideAI](https://decideai.xyz/) implemented a tool for incremental uploading of models: https://github.com/modclub-app/ic-file-uploader/tree/main.

You can install the tool with

```
cargo install ic-file-uploader
```

Afterwards, execute the `upload-models-to-canister.sh` script, which runs the following commands:
```
dfx canister call backend clear_face_detection_model_bytes
dfx canister call backend clear_face_recognition_model_bytes
ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx
ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx
dfx canister call backend setup_models
```

# Credits 

Thanks to [DecideAI](https://decideai.xyz/) for discussions and providing [ic-file-uploader](https://github.com/modclub-app/ic-file-uploader/tree/main).

