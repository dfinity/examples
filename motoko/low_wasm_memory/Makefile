.PHONY: test

test:
	@echo "=== Test 1: getExecutedFunctionsOrder returns an array ==="
	@result=$$(icp canister call backend getExecutedFunctionsOrder '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
