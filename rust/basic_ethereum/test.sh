#!/usr/bin/env bash
set -e

# The local icp-cli network supports real HTTPS outcalls, so all read operations
# (get_balance, transaction_count) can be tested locally against Ethereum Sepolia.
# send_eth requires a funded canister wallet and is not covered here — see README.

# Create a non-anonymous identity for canister calls that reject the anonymous principal.
# --storage-mode plaintext is required in CI environments without a keyring daemon.
icp identity new test --storage-mode plaintext 2>/dev/null || true
icp identity default test

echo "=== Test 1: ethereum_address returns a valid 0x-prefixed Ethereum address ==="
result=$(icp canister call backend ethereum_address '(null)')
echo "$result"
echo "$result" | grep -qE '"0x[0-9a-fA-F]{40}"' && echo "PASS" || (echo "FAIL" && exit 1)
my_address=$(echo "$result" | grep -oE '0x[0-9a-fA-F]{40}')

echo "=== Test 2: ethereum_address for a specific principal returns a valid address ==="
result=$(icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")')
echo "$result"
echo "$result" | grep -qE '"0x[0-9a-fA-F]{40}"' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: different principals derive different Ethereum addresses ==="
addr2=$(icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")' | grep -oE '0x[0-9a-fA-F]{40}')
echo "  my address: $my_address"
echo "  principal address: $addr2"
[ "$my_address" != "$addr2" ] && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: get_balance returns non-zero balance for a known funded Sepolia address ==="
# 0x378a452B20d1f06008C06c581b1656BdC5313c0C is an address with known Sepolia ETH.
# Asserting > 0 proves the HTTPS outcall to the Ethereum RPC provider works and
# returns real on-chain data.
KNOWN_ADDRESS="0x378a452B20d1f06008C06c581b1656BdC5313c0C"
result=$(icp canister call backend get_balance "(opt \"$KNOWN_ADDRESS\")")
echo "$result"
balance=$(echo "$result" | grep -oE '[0-9]+' | head -1)
[ "$balance" -gt 0 ] && echo "PASS" || (echo "FAIL: expected non-zero Sepolia ETH balance for $KNOWN_ADDRESS" && exit 1)

echo "=== Test 5: transaction_count_with_client returns the nonce (outgoing tx count) via EvmRpcClient ==="
# Demonstrates the high-level evm_rpc_client API — same Sepolia HTTPS outcall as Test 4
# but with automatic cycle management and accepting an Ethereum address directly.
# eth_getTransactionCount returns the nonce: only outgoing transactions are counted.
# The known address has 2 outgoing transactions on Sepolia (nonce = 2).
result=$(icp canister call backend transaction_count_with_client "(opt \"$KNOWN_ADDRESS\", null)")
echo "$result"
count=$(echo "$result" | grep -oE '[0-9]+' | head -1)
[ "$count" -ge 2 ] && echo "PASS" || (echo "FAIL: expected nonce >= 2 for $KNOWN_ADDRESS, got $count" && exit 1)
