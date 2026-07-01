#!/usr/bin/env bash
set -e

echo "=== Test 1: get_exchange_rate returns a numeric rate ==="
result=$(icp canister call backend get_exchange_rate \
  '(record { symbol = "ICP"; class = variant { Cryptocurrency } }, record { symbol = "USD"; class = variant { FiatCurrency } })')
echo "$result"
echo "$result" | grep -qE '^\([0-9]' && \
  echo "PASS" || (echo "FAIL" && exit 1)
