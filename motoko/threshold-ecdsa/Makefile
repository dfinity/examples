.PHONY: test

test:
	@echo "=== Test 1: public_key() returns a hex-encoded public key ==="
	@result=$$(icp canister call backend public_key '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'public_key_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: sign() returns a hex-encoded signature ==="
	@result=$$(icp canister call backend sign '("hello world")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'signature_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
