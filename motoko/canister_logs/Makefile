.PHONY: test

test:
	@echo "=== Test 1/11: print via update call is recorded in logs ==="
	@icp canister call backend print '("print via update")' && \
	  icp canister logs backend | grep -q 'print via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/11: print_query via replicated update call is recorded in logs ==="
	@icp canister call backend print_query '("print via replicated query")' && \
	  icp canister logs backend | grep -q 'print via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/11: trap via update call is recorded in logs ==="
	@icp canister call backend trap '("trap via update")' 2>/dev/null || true
	@icp canister logs backend | grep -q 'trap via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4/11: trap logs 'right before trap' message before trapping ==="
	@icp canister logs backend | grep -q 'right before trap' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5/11: trap_query via update call is recorded in logs ==="
	@icp canister call backend trap_query '("trap via replicated query")' 2>/dev/null || true
	@icp canister logs backend | grep -q 'trap via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6/11: trap_query logs 'right before trap_query' before trapping ==="
	@icp canister logs backend | grep -q 'right before trap_query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7/11: memory_oob trap is recorded in logs ==="
	@icp canister call backend memory_oob '()' 2>/dev/null || true
	@icp canister logs backend | grep -q 'Region error: range out of bounds' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8/11: memory_oob logs 'right before memory out of bounds' ==="
	@icp canister logs backend | grep -q 'right before memory out of bounds' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 9/11: timer trap is recorded in logs (waiting up to 10s) ==="
	@sleep 10 && \
	  icp canister logs backend | grep -q 'timer trap' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 10/11: raw_rand returns a blob ==="
	@result=$$(icp canister call backend raw_rand '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 11/11: raw_rand success is recorded in logs ==="
	@icp canister logs backend | grep -q 'ic.raw_rand() call succeeded' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
