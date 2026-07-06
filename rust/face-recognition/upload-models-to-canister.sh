#!/bin/bash
set -e

# Auto-download the face detection model if not already present.
if [ ! -f "version-RFB-320.onnx" ]; then
    echo "Face detection model not found — downloading..."
    bash download-face-detection-model.sh
fi

# Auto-generate the face recognition model if not already present.
if [ ! -f "face-recognition.onnx" ]; then
    # torch only supports Python 3.9-3.12; 3.13+ has no wheels yet.
    PYTHON=""
    for py in python3.12 python3.11 python3.10 python3.9 python3; do
        if which "$py" >/dev/null 2>&1; then
            minor=$("$py" -c "import sys; print(sys.version_info.minor)" 2>/dev/null || echo "0")
            if [ "$minor" -ge 9 ] && [ "$minor" -le 12 ]; then
                PYTHON="$py"
                break
            fi
        fi
    done

    if [ -n "$PYTHON" ]; then
        echo "Face recognition model not found — generating with $PYTHON (this may take a few minutes)..."
        "$PYTHON" -m pip install --quiet --prefer-binary facenet-pytorch torch onnx certifi
        "$PYTHON" << 'PYEOF'
import os, certifi
# Python from python.org on macOS doesn't ship with root certificates;
# point urllib and requests at the certifi bundle so pretrained weights download.
os.environ['SSL_CERT_FILE'] = certifi.where()
os.environ['REQUESTS_CA_BUNDLE'] = certifi.where()
import torch
import facenet_pytorch
resnet = facenet_pytorch.InceptionResnetV1(pretrained='vggface2').eval()
torch.onnx.export(resnet, torch.randn(1, 3, 160, 160), 'face-recognition.onnx', verbose=False, opset_version=11)
print("face-recognition.onnx generated.")
PYEOF
    else
        echo "Face recognition model not found — skipping upload."
        echo "torch requires Python 3.9-3.12. Install it (e.g. 'brew install python@3.12')"
        echo "or generate face-recognition.onnx manually (see README.md)."
        exit 0
    fi
fi

# Skip if models are already loaded.
# On canister upgrades, post_upgrade auto-reloads models from stable memory so
# models_ready() returns true here without any upload needed.
result=$(icp canister call --query backend models_ready '()' 2>/dev/null || echo "(false)")
if echo "$result" | grep -q 'true'; then
    echo "Models already loaded — skipping upload."
    exit 0
fi

echo "Models not loaded — uploading..."

which ic-file-uploader || cargo install ic-file-uploader

if which jq >/dev/null 2>&1; then
    REPLICA_URL=$(icp network status --json | jq -r '.api_url')
elif which python3 >/dev/null 2>&1; then
    REPLICA_URL=$(icp network status --json | python3 -c "import sys,json; print(json.load(sys.stdin)['api_url'])")
else
    REPLICA_URL="http://localhost:8000"
    echo "Note: jq and python3 not found; using default replica URL $REPLICA_URL"
fi
BACKEND_ID=$(icp canister status backend -i)
echo "Backend canister: $BACKEND_ID"

echo "Uploading face detection model..."
icp canister call backend clear_face_detection_model_bytes '()'
ic-file-uploader -n "$REPLICA_URL" "$BACKEND_ID" append_face_detection_model_bytes version-RFB-320.onnx

echo "Uploading face recognition model..."
icp canister call backend clear_face_recognition_model_bytes '()'
ic-file-uploader -n "$REPLICA_URL" "$BACKEND_ID" append_face_recognition_model_bytes face-recognition.onnx

echo "Running setup_models..."
setup_result=$(icp canister call backend setup_models '()')
echo "$setup_result"
if echo "$setup_result" | grep -q 'Err'; then
    echo "Error: setup_models failed: $setup_result"
    exit 1
fi

echo "Models uploaded and loaded successfully."
