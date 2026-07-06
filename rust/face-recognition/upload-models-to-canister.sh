#!/bin/bash
set -e

if [ ! -f "version-RFB-320.onnx" ]; then
    echo "Face detection model not found. Run ./download-face-detection-model.sh first."
    exit 1
fi

if [ ! -f "face-recognition.onnx" ]; then
    echo "Face recognition model not found. Follow the 'Face recognition' section in README.md to generate face-recognition.onnx."
    exit 1
fi

which ic-file-uploader || cargo install ic-file-uploader

icp canister call backend clear_face_detection_model_bytes '()'
icp canister call backend clear_face_recognition_model_bytes '()'
ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx
ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx
icp canister call backend setup_models '()'
