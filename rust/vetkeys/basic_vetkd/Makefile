.PHONY: test

test:
	@echo "--- Testing basic_vetkd: symmetric key verification key ---"
	@result=$$(icp canister call backend symmetric_key_verification_key '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
	@echo "--- Testing basic_vetkd: IBE encryption key ---"
	@result=$$(icp canister call backend ibe_encryption_key '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
