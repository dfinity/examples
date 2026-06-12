.PHONY: test

test:
	@echo "=== Setup: fund the backend canister with ICP ==="
	@backend_principal=$$(icp canister status backend --id-only) && \
	  echo "Backend principal: $$backend_principal" && \
	  icp token transfer 2 "$$backend_principal"

	@echo "=== Test 1: transfer ICP from backend canister to the caller ==="
	@recipient=$$(icp identity principal) && \
	  echo "Recipient principal: $$recipient" && \
	  result=$$(icp canister call backend transfer "(record { amount = record { e8s = 100_000_000 : nat64 }; toPrincipal = principal \"$$recipient\"; toSubaccount = null })") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '#ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
