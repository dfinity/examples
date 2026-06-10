.PHONY: test

test:
	@echo "=== Test 1/9: print via update call is recorded in logs ==="
	@icp canister call backend print '("print via update")' && \
	  icp canister logs backend | grep -q 'print via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/9: print_query via replicated update call is recorded in logs ==="
	@icp canister call backend print_query '("print via replicated query")' && \
	  icp canister logs backend | grep -q 'print via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/9: trap via update call is recorded in logs ==="
	@icp canister call backend trap '("trap via update")' 2>/dev/null || true
	@icp canister logs backend | grep -q 'trap via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4/9: trap logs 'right before trap' message before trapping ==="
	@icp canister logs backend | grep -q 'right before trap' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5/9: trap_query via update call is recorded in logs ==="
	@icp canister call backend trap_query '("trap via replicated query")' 2>/dev/null || true
	@icp canister logs backend | grep -q 'trap via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6/9: memory_oob trap is recorded in logs ==="
	@icp canister call backend memory_oob '()' 2>/dev/null || true
	@icp canister logs backend | grep -q 'Region error: range out of bounds' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7/9: timer trap is recorded in logs (waiting up to 10s) ==="
	@sleep 10 && \
	  icp canister logs backend | grep -q 'timer trap' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8/9: raw_rand returns a blob ==="
	@result=$$(icp canister call backend raw_rand '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 9/9: raw_rand success is recorded in logs ==="
	@icp canister logs backend | grep -q 'ic.raw_rand() call succeeded' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
