#!/bin/bash
set -e

icp canister call backend clear_face_detection_model_bytes '()'
icp canister call backend clear_face_recognition_model_bytes '()'
ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx
ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx
icp canister call backend setup_models '()'
