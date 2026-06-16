.PHONY: test

test:
	@echo "=== Test 1: print via update call is recorded in logs ==="
	@icp canister call backend print '("print via update")' && \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'print via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: print via replicated query call is recorded in logs ==="
	@icp canister call --update backend print_query '("print via replicated query")' && \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'print via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: print via non-replicated query should NOT be recorded in logs ==="
	@icp canister call --query backend print_query '("print via non-replicated query")'; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  ! echo "$$result" | grep -q 'print via non-replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: trap via update call is recorded in logs ==="
	@icp canister call backend trap '("trap via update")' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'trap via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: trap via replicated query call is recorded in logs ==="
	@icp canister call --update backend trap_query '("trap via replicated query")' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'trap via replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: trap via non-replicated query should NOT be recorded in logs ==="
	@icp canister call --query backend trap_query '("trap via non-replicated query")' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  ! echo "$$result" | grep -q 'trap via non-replicated query' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: panic via update call is recorded in logs ==="
	@icp canister call backend panic '("panic via update")' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'panic via update' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8: memory out of bounds call is recorded in logs ==="
	@icp canister call backend memory_oob '()' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'stable memory out of bounds' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 9: failed unwrap call is recorded in logs ==="
	@icp canister call backend failed_unwrap '()' 2>/dev/null; \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q "Result::unwrap()" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 10: Polling for timer trap (up to 60s) ==="
	@secs=0; \
	while [ $$secs -lt 60 ]; do \
	  result=$$(icp canister logs backend); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'timer trap' && echo "PASS" && exit 0; \
	  sleep 3; secs=$$((secs + 3)); \
	done; \
	echo "FAIL: timer trap not recorded within 60s"; exit 1

	@echo "=== Test 11: raw_rand call is recorded in logs ==="
	@icp canister call backend raw_rand '()' && \
	  result=$$(icp canister logs backend) && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ic.raw_rand() call succeeded' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
