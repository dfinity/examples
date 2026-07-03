#!/usr/bin/env bash
set -e

trap 'rm -f /tmp/stake-neuron-test.pem; icp identity delete stake-neuron-test 2>/dev/null || true' EXIT

cargo build --release -q

BINARY=./target/release/stake_neuron_from_cli
URL=http://localhost:8000

# Create a plaintext test identity and export its PEM.
# nns: true in icp.yaml seeds the default icp-cli identity with ICP at startup.
icp identity new --storage plaintext stake-neuron-test 2>/dev/null || true
icp identity export stake-neuron-test > /tmp/stake-neuron-test.pem
PRINCIPAL=$(icp identity principal --identity stake-neuron-test)
echo "Test identity: $PRINCIPAL"

echo "=== Test 1: subaccount is deterministic for the same (identity, nonce) pair ==="
# --compute-only derives the subaccount without making any IC calls or transfers.
sub1=$("$BINARY" --compute-only --identity /tmp/stake-neuron-test.pem --nonce 0 | grep "Subaccount" | awk '{print $3}')
sub2=$("$BINARY" --compute-only --identity /tmp/stake-neuron-test.pem --nonce 0 | grep "Subaccount" | awk '{print $3}')
echo "Subaccount: $sub1"
[ "$sub1" = "$sub2" ] && echo "PASS" || (echo "FAIL: subaccount is not deterministic" && exit 1)

echo "=== Test 2: different nonces produce different subaccounts ==="
sub_n0=$("$BINARY" --compute-only --identity /tmp/stake-neuron-test.pem --nonce 0 | grep "Subaccount" | awk '{print $3}')
sub_n1=$("$BINARY" --compute-only --identity /tmp/stake-neuron-test.pem --nonce 1 | grep "Subaccount" | awk '{print $3}')
echo "Nonce 0: $sub_n0"
echo "Nonce 1: $sub_n1"
[ "$sub_n0" != "$sub_n1" ] && echo "PASS" || (echo "FAIL: different nonces produced the same subaccount" && exit 1)

echo "=== Test 3: stake_neuron rejects amount below 1 ICP minimum ==="
"$BINARY" --url "$URL" --identity /tmp/stake-neuron-test.pem --amount-e8s 99999999 --nonce 0 2>&1 | \
  grep -q "100_000_000" && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Setup: fund test identity with 2 ICP ==="
icp token transfer 2 "$PRINCIPAL"

echo "=== Test 4: stake 1 ICP and receive a neuron ID ==="
output=$("$BINARY" --url "$URL" --identity /tmp/stake-neuron-test.pem --amount-e8s 100000000 --nonce 0)
echo "$output"
echo "$output" | grep -q "Neuron ID" && echo "PASS" || (echo "FAIL" && exit 1)
