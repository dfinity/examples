#!/usr/bin/env bash
set -e

echo "--- Testing basic_bls_signing: verification key ---"
result=$(icp canister call backend get_my_verification_key '()') && \
  echo "$result" | head -1 && \
  echo "$result" | grep -q 'blob' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Testing basic_bls_signing: my signatures (query) ---"
result=$(icp canister call backend get_my_signatures '()' --query) && \
  echo "$result" && \
  echo "$result" | grep -q 'vec' && \
  echo "PASS" || (echo "FAIL" && exit 1)
