#!/usr/bin/env bash
set -e

echo "--- Testing encrypted_notes: whoami ---"
result=$(icp canister call backend whoami '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Testing encrypted_notes: get_notes (empty) ---"
result=$(icp canister call backend get_notes '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'vec' && \
  echo "PASS" || (echo "FAIL" && exit 1)
