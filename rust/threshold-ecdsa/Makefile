.PHONY: test

test:
	@echo "=== Test 1/3: public_key() returns a hex-encoded public key ==="
	@result=$$(icp canister call backend public_key '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'public_key_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/3: sign() returns a hex-encoded signature ==="
	@result=$$(icp canister call backend sign '("hello world")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'signature_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/3: signature verifies cryptographically (secp256k1) ==="
	@npm install --silent
	@chmod +x test.sh && ./test.sh "hello world" && echo "PASS" || (echo "FAIL" && exit 1)