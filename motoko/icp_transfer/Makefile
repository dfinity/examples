.PHONY: test

ICP_LEDGER := ryjl3-tyaaa-aaaaa-aaaba-cai

# Tests use a single shell block so all variables and functions stay in scope.
# Balance checks are delta-based so the tests are idempotent across re-runs.
test:
	@set -e; \
	  \
	  backend=$$(icp canister status backend -i); \
	  recipient=$$(icp identity principal); \
	  echo "Backend:   $$backend"; \
	  echo "Recipient: $$recipient"; \
	  \
	  get_balance() { \
	    icp token balance -q --of-principal "$$1" \
	      | awk '{printf "%.0f", $$1 * 100000000}'; \
	  }; \
	  \
	  echo "=== Test 1: toAccountId converts a principal to its AccountIdentifier blob ==="; \
	  account_id=$$(icp canister call --query backend toAccountId \
	    "(principal \"$$recipient\", null)"); \
	  echo "AccountIdentifier: $$account_id"; \
	  echo "$$account_id" | grep -q 'blob' || (echo "FAIL: expected blob output" && exit 1); \
	  echo "PASS"; \
	  \
	  echo "=== Setup: fund backend with 4 ICP (2 per transfer call) ==="; \
	  icp token transfer 4 "$$backend"; \
	  \
	  echo "=== Test 2: transferToPrincipal — backend loses 100_010_000 e8s, recipient gains 100_000_000 e8s ==="; \
	  backend_before=$$(get_balance "$$backend"); \
	  recipient_before=$$(get_balance "$$recipient"); \
	  result=$$(icp canister call backend transferToPrincipal \
	    "(record { e8s = 100_000_000 : nat64 }, principal \"$$recipient\", null)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'ok' || (echo "FAIL: transfer rejected" && exit 1); \
	  backend_after=$$(get_balance "$$backend"); \
	  recipient_after=$$(get_balance "$$recipient"); \
	  backend_delta=$$((backend_after - backend_before)); \
	  recipient_delta=$$((recipient_after - recipient_before)); \
	  echo "Backend:   before=$$backend_before after=$$backend_after delta=$$backend_delta"; \
	  echo "Recipient: before=$$recipient_before after=$$recipient_after delta=$$recipient_delta"; \
	  [ "$$backend_delta" -eq -100010000 ] || (echo "FAIL: expected backend delta -100010000 e8s (amount + fee)" && exit 1); \
	  [ "$$recipient_delta" -eq 100000000 ] || (echo "FAIL: expected recipient delta +100000000 e8s" && exit 1); \
	  echo "PASS"; \
	  \
	  echo "=== Test 3: transferToAccountId with the blob from toAccountId — same recipient ==="; \
	  account_id_blob=$$(echo "$$account_id" | grep -oE 'blob "[^"]*"'); \
	  backend_before=$$(get_balance "$$backend"); \
	  recipient_before=$$(get_balance "$$recipient"); \
	  result=$$(icp canister call backend transferToAccountId \
	    "(record { e8s = 100_000_000 : nat64 }, $$account_id_blob)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'ok' || (echo "FAIL: transfer rejected" && exit 1); \
	  backend_after=$$(get_balance "$$backend"); \
	  recipient_after=$$(get_balance "$$recipient"); \
	  backend_delta=$$((backend_after - backend_before)); \
	  recipient_delta=$$((recipient_after - recipient_before)); \
	  echo "Backend:   before=$$backend_before after=$$backend_after delta=$$backend_delta"; \
	  echo "Recipient: before=$$recipient_before after=$$recipient_after delta=$$recipient_delta"; \
	  [ "$$backend_delta" -eq -100010000 ] || (echo "FAIL: expected backend delta -100010000 e8s" && exit 1); \
	  [ "$$recipient_delta" -eq 100000000 ] || (echo "FAIL: expected recipient delta +100000000 e8s" && exit 1); \
	  echo "PASS — same recipient reached via AccountIdentifier blob as via principal"
