.PHONY: test

test:
	@echo "=== Test 1: set stores a value ==="
	@result=$$(icp canister call backend set '(42 : nat32)') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: inc returns the incremented value (proves set(42) was stored) ==="
	@result=$$(icp canister call backend inc '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '43' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: get returns a record with a certificate ==="
	@result=$$(icp canister call --query backend get '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'certificate' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: inc increments again and returns the new value ==="
	@result=$$(icp canister call backend inc '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '44' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
