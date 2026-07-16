#!/usr/bin/env bash
set -e

echo "--- Testing password_manager_with_metadata: vetkey verification key ---"
result=$(icp canister call backend get_vetkey_verification_key '()') && \
  echo "$result" | head -1 && \
  echo "$result" | grep -q 'blob' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Testing password_manager_with_metadata: owned map names (query) ---"
result=$(icp canister call backend get_owned_non_empty_map_names '()' --query) && \
  echo "$result" && \
  echo "$result" | grep -q 'vec' && \
  echo "PASS" || (echo "FAIL" && exit 1)
