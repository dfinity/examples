.PHONY: test upload-models

test:
	@echo "=== Test 1: clear_face_detection_model_bytes returns unit ==="
	@result=$$(icp canister call backend clear_face_detection_model_bytes '()') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: clear_face_recognition_model_bytes returns unit ==="
	@result=$$(icp canister call backend clear_face_recognition_model_bytes '()') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: append_face_detection_model_bytes accepts bytes ==="
	@result=$$(icp canister call backend append_face_detection_model_bytes '(blob "\00\01\02")') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: run_detection returns Err when model is not set up ==="
	@result=$$(icp canister call --query backend run_detection '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Err' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

# Upload ONNX model files to the canister incrementally.
# Requires: ic-file-uploader (cargo install ic-file-uploader)
# First download the models: ./download-face-detection-model.sh
# and obtain face-recognition.onnx (see README), then run: make upload-models
upload-models:
	icp canister call backend clear_face_detection_model_bytes '()'
	icp canister call backend clear_face_recognition_model_bytes '()'
	ic-file-uploader backend append_face_detection_model_bytes version-RFB-320.onnx
	ic-file-uploader backend append_face_recognition_model_bytes face-recognition.onnx
	icp canister call backend setup_models '()'
