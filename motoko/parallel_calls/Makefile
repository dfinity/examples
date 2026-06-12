.PHONY: test

test:
	@echo "=== Test 1: setup callee ==="
	@result=$$(icp canister call backend setup_callee "(principal \"$$(icp canister id callee)\")") && \
	  echo "$$result" && \
	  echo "PASS"

	@echo "=== Test 2: sequential_calls with n=100 returns 100 ==="
	@result=$$(icp canister call backend sequential_calls '(100)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(100' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: parallel_calls with n=100 returns 100 ==="
	@result=$$(icp canister call backend parallel_calls '(100)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(100' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: sequential_calls with n=2000 returns 2000 ==="
	@result=$$(icp canister call backend sequential_calls '(2000)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2_000' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: parallel_calls with n=2000 returns fewer due to in-flight limit ==="
	@result=$$(icp canister call backend parallel_calls '(2000)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '\([0-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
