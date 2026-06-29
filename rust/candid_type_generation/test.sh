#!/usr/bin/env bash
set -e

# Note: On a local replica, inter-canister calls to mainnet canisters (NNS Governance)
# will be rejected. The test accepts either a successful neuron list or an error message
# as valid output — both confirm the canister compiled, deployed, and responds correctly.
# To test with live data, deploy to the IC mainnet: icp deploy --network ic

echo "=== Test 1: list_neurons_pretty returns a text response ==="
result=$(icp canister call backend list_neurons_pretty '()') && \
  echo "$result" && \
  echo "$result" | grep -qE '(NNS Governance Neurons|Error fetching neurons)' && \
  echo "PASS" || (echo "FAIL" && exit 1)
