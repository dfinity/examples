.PHONY: test topup

topup:
	icp canister top-up --amount 30t backend

test:
	@echo "=== Test 1: put inserts a key-value pair ==="
	@icp canister call backend put '(0, "zero")' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get (composite query) retrieves the stored value ==="
	@# This exercises the composite query: Map.get calls Bucket.get on a child canister.
	@result=$$(icp canister call --query backend get '(0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt "zero"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: getUpdate returns the same value via an update call ==="
	@result=$$(icp canister call backend getUpdate '(0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt "zero"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get returns null for a missing key ==="
	@result=$$(icp canister call --query backend get '(99)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'null' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: composite query routes correctly across all four buckets ==="
	@# Keys 0-3 each map to a different bucket (k % 4): 0→B0, 1→B1, 2→B2, 3→B3.
	@# The composite query must call the right child canister for each key.
	@icp canister call backend put '(1, "one")' && \
	  icp canister call backend put '(2, "two")' && \
	  icp canister call backend put '(3, "three")' && \
	  result0=$$(icp canister call --query backend get '(0)') && \
	  result1=$$(icp canister call --query backend get '(1)') && \
	  result2=$$(icp canister call --query backend get '(2)') && \
	  result3=$$(icp canister call --query backend get '(3)') && \
	  echo "$$result0" && echo "$$result1" && echo "$$result2" && echo "$$result3" && \
	  echo "$$result0" | grep -q 'opt "zero"' && \
	  echo "$$result1" | grep -q 'opt "one"' && \
	  echo "$$result2" | grep -q 'opt "two"' && \
	  echo "$$result3" | grep -q 'opt "three"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
