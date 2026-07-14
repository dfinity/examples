#!/usr/bin/env bash
set -e

echo "=== Test 1: clear_face_detection_model_bytes returns unit ==="
result=$(icp canister call backend clear_face_detection_model_bytes '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: clear_face_recognition_model_bytes returns unit ==="
result=$(icp canister call backend clear_face_recognition_model_bytes '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: append_face_detection_model_bytes accepts bytes ==="
result=$(icp canister call backend append_face_detection_model_bytes '(blob "\00\01\02")') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: run_detection result depends on whether models are loaded ==="
# models_ready() is false in CI (no model files) and true locally after sync.
# Clearing the byte storage does not unload in-memory models — so locally
# models remain loaded and detection succeeds; in CI it returns Err.
ready=$(icp canister call --query backend models_ready '()' | grep -c 'true' || true)
result=$(icp canister call --query backend run_detection '()')
echo "$result"
if [ "$ready" -gt 0 ]; then
  echo "$result" | grep -q 'Ok' && echo "PASS (models loaded — detection succeeded)" || (echo "FAIL" && exit 1)
else
  echo "$result" | grep -q 'Err' && echo "PASS (no models — expected error returned)" || (echo "FAIL" && exit 1)
fi
