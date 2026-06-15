.PHONY: test

test:
	@echo "=== Test 1: set stores a value ==="
	@icp canister call backend set '(42 : nat32)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get returns the stored value with a certificate ==="
	@icp canister call --query backend get '()' | tee /dev/stderr | grep -q 'value = 42' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: inc increments and returns the new value ==="
	@result=$$(icp canister call backend inc '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '43' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get reflects the incremented value ==="
	@icp canister call --query backend get '()' | tee /dev/stderr | grep -q 'value = 43' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
