#!/bin/bash
set -e

# Auto-download the face detection model if not already present.
if [ ! -f "version-RFB-320.onnx" ]; then
    echo "Face detection model not found — downloading..."
    bash download-face-detection-model.sh
fi

# Auto-generate the face recognition model if not already present.
if [ ! -f "face-recognition.onnx" ]; then
    if which python3 >/dev/null 2>&1; then
        echo "Face recognition model not found — generating (this may take a few minutes)..."
        python3 -m pip install --quiet facenet-pytorch torch onnx
        python3 << 'PYEOF'
import torch
import facenet_pytorch
resnet = facenet_pytorch.InceptionResnetV1(pretrained='vggface2').eval()
torch.onnx.export(resnet, torch.randn(1, 3, 160, 160), 'face-recognition.onnx', verbose=False, opset_version=11)
print("face-recognition.onnx generated.")
PYEOF
    else
        echo "Face recognition model not found and python3 is not available — skipping upload."
        echo "Install Python3 or generate face-recognition.onnx manually (see README.md)."
        exit 0
    fi
fi

# Skip if models are already loaded (e.g. on redeployment without reinstall).
result=$(icp canister call --query backend models_ready '()' 2>/dev/null || echo "(false)")
echo "models_ready: $result"
if echo "$result" | grep -q 'true'; then
    echo "Models already loaded — skipping upload."
    exit 0
fi

which ic-file-uploader || cargo install ic-file-uploader

echo "Uploading face detection model..."
icp canister call backend clear_face_detection_model_bytes '()'
ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx

echo "Uploading face recognition model..."
icp canister call backend clear_face_recognition_model_bytes '()'
ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx

echo "Running setup_models..."
setup_result=$(icp canister call backend setup_models '()')
echo "$setup_result"
if echo "$setup_result" | grep -q 'Err'; then
    echo "Error: setup_models failed: $setup_result"
    exit 1
fi

echo "Models uploaded and loaded successfully."
