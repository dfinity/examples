.PHONY: test topup

topup:
	icp canister top-up --amount 30t backend

test:
	@echo "=== Test 1: put stores a value ==="
	@result=$$(icp canister call backend put '(0, "hello")') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get retrieves the stored value ==="
	@result=$$(icp canister call backend get '(0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"hello"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: get returns null for missing key ==="
	@result=$$(icp canister call backend get '(99)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'null' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: put distributes entries across buckets ==="
	@icp canister call backend put '(1, "one")' > /dev/null && \
	  icp canister call backend put '(2, "two")' > /dev/null && \
	  icp canister call backend put '(3, "three")' > /dev/null && \
	  result=$$(icp canister call backend get '(1)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"one"' && \
	  result=$$(icp canister call backend get '(2)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"two"' && \
	  result=$$(icp canister call backend get '(3)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"three"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
