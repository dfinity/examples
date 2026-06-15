.PHONY: test

test:
	@echo "=== Test 1: get_balance returns the canister's cycle balance ==="
	@result=$$(icp canister call --query backend get_balance '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '[0-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: send_cycles forwards 5M cycles — all accepted, refunded = 0 ==="
	@BACKEND_ID=$$(icp canister status backend -i) && \
	  result=$$(icp canister call backend send_cycles "(func \"$$BACKEND_ID\".\"accept_cycles\", 5_000_000)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'refunded = 0' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: send_cycles forwards 15M cycles — 5M refunded (over the 10M limit) ==="
	@BACKEND_ID=$$(icp canister status backend -i) && \
	  result=$$(icp canister call backend send_cycles "(func \"$$BACKEND_ID\".\"accept_cycles\", 15_000_000)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE 'refunded = [1-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
