#!/usr/bin/env bash
set -e

echo "--- Testing basic_ibe: IBE public key ---"
result=$(icp canister call backend get_ibe_public_key '()') && \
  echo "$result" | head -1 && \
  echo "$result" | grep -q 'blob' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Testing basic_ibe: empty inbox query ---"
result=$(icp canister call backend get_my_messages '()' --query) && \
  echo "$result" && \
  echo "$result" | grep -q 'messages' && \
  echo "PASS" || (echo "FAIL" && exit 1)
