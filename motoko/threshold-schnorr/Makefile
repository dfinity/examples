.PHONY: test

test:
	@echo "=== Test 1/4: public_key returns hex for bip340secp256k1 ==="
	@result=$$(icp canister call backend public_key '(variant { bip340secp256k1 })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'public_key_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/4: public_key returns hex for ed25519 ==="
	@result=$$(icp canister call backend public_key '(variant { ed25519 })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'public_key_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/4: sign returns signature_hex for bip340secp256k1 ==="
	@result=$$(icp canister call backend sign '("hello world", variant { bip340secp256k1 }, null)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'signature_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4/4: sign returns signature_hex for ed25519 ==="
	@result=$$(icp canister call backend sign '("hello world", variant { ed25519 }, null)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'signature_hex' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
