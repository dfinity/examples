.PHONY: test

test:
	@echo "=== Test 1: wallet_balance returns a nat ==="
	@result=$$(icp canister call backend wallet_balance '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: wallet_receive accepts cycles (returns accepted count) ==="
	@result=$$(icp canister call backend wallet_receive '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'accepted' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: transfer to own wallet_receive returns refunded field ==="
	@BACKEND_ID=$$(icp canister status backend -i) && \
	  result=$$(icp canister call backend transfer "(func \"$$BACKEND_ID\".\"wallet_receive\", 0)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'refunded' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
