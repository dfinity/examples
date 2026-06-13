IMAGE_NAME = icp-cli-network-launcher-bitcoin
# Find the running container built from our custom image
BITCOIN_CONTAINER = $(shell docker ps --filter "ancestor=$(IMAGE_NAME)" --format "{{.ID}}" | head -1)

.PHONY: build-image test topup

build-image:
	# Because we're building off of :latest, use --pull to fetch the latest image
	# Remove `--pull` if the Dockerfile is updated to pin the version of the base
	# image
	docker build --pull -t $(IMAGE_NAME) .

topup:
	icp canister top-up --amount 30t backend

test:
	@echo "=== Test 1: get_p2pkh_address returns a valid Bitcoin address ==="
	@result=$$(icp canister call backend get_p2pkh_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get_p2tr_key_only_address returns a valid Bitcoin address ==="
	@result=$$(icp canister call backend get_p2tr_key_only_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: get_p2tr_address returns a valid Bitcoin address ==="
	@result=$$(icp canister call backend get_p2tr_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get_current_fee_percentiles returns a vec ==="
	@result=$$(icp canister call backend get_current_fee_percentiles '()') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Mining 101 blocks to fund test address ==="
	@addr=$$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
	  docker exec $(BITCOIN_CONTAINER) bitcoin-cli -regtest \
	    -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
	    generatetoaddress 101 "$$addr" > /dev/null && \
	  echo "mined 101 blocks to $$addr"

	@echo "=== Waiting for IC to sync Bitcoin blocks ==="
	@sleep 5

	@echo "=== Test 5: get_balance returns non-zero after mining ==="
	@addr=$$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
	  result=$$(icp canister call backend get_balance "(\"$$addr\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '[1-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: get_utxos returns synced chain state after mining ==="
	@addr=$$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"') && \
	  result=$$(icp canister call backend get_utxos "(\"$$addr\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'tip_height = 101' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
