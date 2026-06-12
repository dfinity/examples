.PHONY: test

test:
	@echo "=== Test 1: greet returns greeting ==="
	@result=$$(icp canister call backend greet '("World")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"Hello, World!"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
