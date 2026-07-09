#!/usr/bin/env bash
set -e

IMAGE_NAME=icp-cli-network-launcher-bitcoin
# Find the running container built from our custom image
BITCOIN_CONTAINER=$(docker ps --filter "ancestor=${IMAGE_NAME}" --format "{{.ID}}" | head -1)

echo "=== Test 1: get_p2pkh_address returns a valid Bitcoin address ==="
result=$(icp canister call backend get_p2pkh_address '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: get_p2wpkh_address returns a valid Bitcoin address ==="
result=$(icp canister call backend get_p2wpkh_address '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: get_p2tr_key_path_only_address returns a valid Bitcoin address ==="
result=$(icp canister call backend get_p2tr_key_path_only_address '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: get_p2tr_script_path_enabled_address returns a valid Bitcoin address ==="
result=$(icp canister call backend get_p2tr_script_path_enabled_address '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: get_current_fee_percentiles returns a vec ==="
result=$(icp canister call backend get_current_fee_percentiles '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Mining 101 blocks to fund test address ==="
[ -n "$BITCOIN_CONTAINER" ] || (echo "ERROR: network launcher container not running — run 'icp network start -d' first" && exit 1)
addr=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
  docker exec "$BITCOIN_CONTAINER" bitcoin-cli -regtest \
    -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
    generatetoaddress 101 "$addr" > /dev/null && \
  echo "mined 101 blocks to $addr"

echo "=== Waiting for IC to sync Bitcoin blocks ==="
sleep 5

echo "=== Test 6: get_balance returns non-zero after mining ==="
addr=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
  result=$(icp canister call backend get_balance "(\"$addr\")") && \
  echo "$result" && \
  echo "$result" | grep -qE '[1-9]' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: get_utxos returns synced chain state after mining ==="
addr=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
  result=$(icp canister call backend get_utxos "(\"$addr\")") && \
  echo "$result" && \
  echo "$result" | grep -qE 'tip_height = [1-9][0-9]*' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 8: get_blockchain_info returns tip_height ==="
result=$(icp canister call backend get_blockchain_info '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'height' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 9: get_block_headers returns headers ==="
result=$(icp canister call backend get_block_headers '(0: nat32, null)') && \
  echo "$result" && \
  echo "$result" | grep -q 'tip_height' && \
  echo "PASS" || (echo "FAIL" && exit 1)
