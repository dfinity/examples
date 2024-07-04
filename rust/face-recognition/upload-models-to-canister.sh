#!/bin/bash
set -e

dfx canister call backend clear_face_detection_model_bytes
dfx canister call backend clear_face_recognition_model_bytes
upload_byte_file backend append_face_detection_model_bytes . version-RFB-320.onnx
upload_byte_file backend append_face_recognition_model_bytes . face-recognition.onnx
dfx canister call backend setup_models
