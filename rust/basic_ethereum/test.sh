#!/usr/bin/env bash
set -e

# The local icp-cli network supports real HTTPS outcalls, so all read operations
# (get_balance, transaction_count) can be tested locally against Ethereum Sepolia.
# send_eth requires a funded canister wallet and is not covered here — see README.

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
echo "  canister address: $my_address"
echo "  principal address: $addr2"
[ "$my_address" != "$addr2" ] && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: get_balance makes a real HTTPS outcall to Ethereum Sepolia and returns a valid Nat ==="
# Uses the canister's own Sepolia address. Balance may be 0 but the call must succeed,
# proving the EVM RPC canister's HTTPS outcall to PublicNode/Ankr works locally.
result=$(icp canister call backend get_balance "(opt \"$my_address\")")
echo "$result"
echo "$result" | grep -qE '\([0-9]+ : nat\)' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: transaction_count_with_client returns a valid Nat via EvmRpcClient ==="
# Demonstrates the high-level evm_rpc_client API — same HTTPS outcall, no manual cycle management.
result=$(icp canister call backend transaction_count_with_client "(null, null)")
echo "$result"
echo "$result" | grep -qE '\([0-9]+ : nat\)' && echo "PASS" || (echo "FAIL" && exit 1)
