.PHONY: test

test:
	@echo "=== Test 1: set stores a value ==="
	@result=$$(icp canister call backend set '(42 : nat32)') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get returns the value that was set ==="
	@result=$$(icp canister call --query backend get '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '42' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: inc increments and returns the new value ==="
	@result=$$(icp canister call backend inc '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '43' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get reflects the incremented value ==="
	@result=$$(icp canister call --query backend get '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '43' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
