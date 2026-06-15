.PHONY: test topup

topup:
	icp canister top-up --amount 30t backend

test:
	@echo "=== Test 1: put inserts a key-value pair ==="
	@result=$$(icp canister call backend put '(0, "hello")') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get (composite query) returns the stored value ==="
	@result=$$(icp canister call --query backend get '(0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt "hello"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: getUpdate returns the stored value ==="
	@result=$$(icp canister call backend getUpdate '(0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt "hello"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get returns null for a missing key ==="
	@result=$$(icp canister call --query backend get '(1)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'null' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: put multiple entries and verify distribution across buckets ==="
	@icp canister call backend put '(4, "four")' && \
	  icp canister call backend put '(8, "eight")' && \
	  result=$$(icp canister call --query backend get '(4)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt "four"' && \
	  result2=$$(icp canister call --query backend get '(8)') && \
	  echo "$$result2" && \
	  echo "$$result2" | grep -q 'opt "eight"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
