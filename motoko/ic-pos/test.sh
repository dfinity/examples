#!/usr/bin/env bash
set -e

# End-to-end test of the ic-pos payment flow using icp-cli only.
# Prerequisites: a running local network and `bash deploy.sh` already run
# (which creates the pre-funded ic-pos-dev identity).

icp identity new ic-pos-merchant --storage plaintext 2>/dev/null || true
MERCHANT=$(icp identity principal --identity ic-pos-merchant)

balance_of() {
  icp canister call icrc1_ledger icrc1_balance_of "(record { owner = principal \"$MERCHANT\" })" |
    grep -oE '[0-9_]+' | tr -d '_'
}

echo "=== Test 1: configure a merchant ==="
icp canister call icpos updateMerchant "(record { \
  name = \"Test Shop\"; \
  email_notifications = true; email_address = \"shop@example.com\"; \
  phone_notifications = false; phone_number = \"\" })" --identity ic-pos-merchant
result=$(icp canister call icpos getMerchant "()" --identity ic-pos-merchant)
echo "$result"
echo "$result" | grep -q "Test Shop" && echo "PASS" || { echo "FAIL: merchant not stored"; exit 1; }

echo "=== Test 2: pay the merchant (real transfer from the pre-funded dev identity) ==="
before=$(balance_of)
icp canister call icrc1_ledger icrc1_transfer "(record { \
  to = record { owner = principal \"$MERCHANT\" }; \
  amount = 500_000 : nat })" --identity ic-pos-dev

echo "=== Test 3: merchant balance increased by the payment amount ==="
after=$(balance_of)
echo "balance: $before -> $after"
[ "$((after - before))" -eq 500000 ] && echo "PASS" || { echo "FAIL: expected +500000, got +$((after - before))"; exit 1; }

# The backend's global timer scans the ledger and, for a merchant with
# notifications enabled, emits a would-be-notification entry to its canister
# logs (see `icp canister logs icpos`). That is timing-dependent (the timer runs
# every 20s), so it is not asserted here.
echo "All tests passed."
