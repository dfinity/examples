#!/usr/bin/env bash
set -e

echo "=== Test 1: list_neurons_pretty returns a JSON response from NNS Governance ==="
result=$(icp canister call backend list_neurons_pretty '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'NNS Governance Neurons' && \
  echo "PASS" || (echo "FAIL" && exit 1)
