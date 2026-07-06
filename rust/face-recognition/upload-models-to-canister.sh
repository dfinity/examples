#!/bin/bash
set -e

# Skip gracefully if model files haven't been downloaded/generated yet.
# This keeps icp deploy working in CI without the large model files.
if [ ! -f "version-RFB-320.onnx" ] || [ ! -f "face-recognition.onnx" ]; then
    echo "Model files not found — skipping upload."
    echo "To enable face recognition, first:"
    echo "  1. Run ./download-face-detection-model.sh"
    echo "  2. Generate face-recognition.onnx (see README.md)"
    echo "  3. Run icp deploy again (or bash upload-models-to-canister.sh)"
    exit 0
fi

# Skip if models are already loaded (e.g. on redeployment without reinstall).
result=$(icp canister call --query backend models_ready '()' 2>/dev/null || echo "(false)")
if echo "$result" | grep -q 'true'; then
    echo "Models already loaded — skipping upload."
    exit 0
fi

which ic-file-uploader || cargo install ic-file-uploader

icp canister call backend clear_face_detection_model_bytes '()'
icp canister call backend clear_face_recognition_model_bytes '()'
ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx
ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx
icp canister call backend setup_models '()'
echo "Models uploaded and loaded successfully."
