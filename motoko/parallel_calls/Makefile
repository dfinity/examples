.PHONY: test

test:
	@echo "=== Test 1: wire caller to callee ==="
	@CALLEE_ID=$$(icp canister status callee -i) && \
	  icp canister call caller setup_callee "(principal \"$$CALLEE_ID\")" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: sequential_calls(100) — all succeed on single subnet ==="
	@result=$$(icp canister call caller sequential_calls '(100)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(100' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: parallel_calls(100) — all succeed (within in-flight limit) ==="
	@result=$$(icp canister call caller parallel_calls '(100)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(100' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: sequential_calls(2000) — all succeed (one in-flight at a time) ==="
	@result=$$(icp canister call caller sequential_calls '(2000)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2_000' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: parallel_calls(2000) — fewer succeed (in-flight limit exceeded) ==="
	@# The replica limits in-flight calls per canister, so most parallel calls fail at n=2000.
	@# Verify we get fewer than 2000 successful calls, confirming the limit was hit.
	@result=$$(icp canister call caller parallel_calls '(2000)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qv '(2_000' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
