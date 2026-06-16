.PHONY: test

# Note: get_balance, transaction_count, and send_eth require live HTTPS outcalls
# to the Ethereum network and cannot be fully tested locally. The tests below
# verify canister deployment and the ethereum_address function, which derives
# an address using threshold ECDSA (available locally).
# For full end-to-end testing, deploy to ICP mainnet with Sepolia testnet.
test:
	@echo "=== Test 1: ethereum_address returns a valid 0x-prefixed Ethereum address ==="
	@result=$$(icp canister call backend ethereum_address '(null)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '"0x[0-9a-fA-F]{40}"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: ethereum_address for a specific principal returns a valid address ==="
	@result=$$(icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '"0x[0-9a-fA-F]{40}"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: different principals derive different Ethereum addresses ==="
	@addr1=$$(icp canister call backend ethereum_address '(null)' | grep -oE '0x[0-9a-fA-F]{40}') && \
	  addr2=$$(icp canister call backend ethereum_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")' | grep -oE '0x[0-9a-fA-F]{40}') && \
	  echo "addr1=$$addr1 addr2=$$addr2" && \
	  [ "$$addr1" != "$$addr2" ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)
