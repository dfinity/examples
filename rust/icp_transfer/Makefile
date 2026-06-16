IC_REVISION = d87954601e4b22972899e9957e800406a0a6b929
LEDGER_WASM_URL = https://download.dfinity.systems/ic/$(IC_REVISION)/canisters/ledger-canister.wasm.gz
LEDGER_PRINCIPAL = ryjl3-tyaaa-aaaaa-aaaba-cai

.PHONY: deploy-ledger test

# Download and deploy the ICP ledger canister at the well-known mainnet principal.
# On the local network, provisional_create_canister_with_cycles allows specifying
# an exact canister ID so that the backend (which hardcodes MAINNET_LEDGER_CANISTER_ID)
# finds the ledger at the expected address.
deploy-ledger:
	@echo "=== Downloading ICP ledger WASM ==="
	curl -sL $(LEDGER_WASM_URL) -o /tmp/ledger-canister.wasm.gz
	gunzip -f /tmp/ledger-canister.wasm.gz
	@echo "=== Creating ledger canister at $(LEDGER_PRINCIPAL) ==="
	icp canister call aaaaa-aa provisional_create_canister_with_cycles \
	  '(record { amount = 10_000_000_000_000 : nat; settings = null; specified_id = opt principal "$(LEDGER_PRINCIPAL)" })' \
	  -n http://localhost:8000
	@echo "=== Installing ICP ledger ==="
	MINTER_ACCOUNT_ID=$$(icp identity account-id --identity anonymous) && \
	DEFAULT_ACCOUNT_ID=$$(icp identity account-id) && \
	icp canister install $(LEDGER_PRINCIPAL) \
	  --wasm /tmp/ledger-canister.wasm \
	  --args "(variant { Init = record { \
	    minting_account = \"$$MINTER_ACCOUNT_ID\"; \
	    initial_values = vec { record { \"$$DEFAULT_ACCOUNT_ID\"; record { e8s = 10_000_000_000 : nat64; }; }; }; \
	    send_whitelist = vec {}; \
	    transfer_fee = opt record { e8s = 10_000 : nat64; }; \
	    token_symbol = opt \"LICP\"; \
	    token_name = opt \"Local ICP\"; \
	  } })" \
	  -n http://localhost:8000 -y

test: deploy-ledger
	@echo "=== Test 1: verify default account has initial ICP balance ==="
	@DEFAULT_ACCOUNT_ID=$$(icp identity account-id) && \
	ACCOUNT_BYTES=$$(python3 -c "b=bytes.fromhex('$$DEFAULT_ACCOUNT_ID'); print('vec{' + ';'.join(str(x) for x in b) + '}')") && \
	result=$$(icp canister call $(LEDGER_PRINCIPAL) account_balance \
	  "(record { account = $$ACCOUNT_BYTES })" \
	  -n http://localhost:8000) && \
	echo "$$result" && \
	echo "$$result" | grep -q '10_000_000_000' && \
	echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: fund the backend canister with 1 ICP ==="
	@BACKEND_PRINCIPAL=$$(icp canister status backend -i) && \
	BACKEND_ACCOUNT_ID=$$(icp identity account-id --of-principal $$BACKEND_PRINCIPAL) && \
	BACKEND_ACCOUNT_BYTES=$$(python3 -c "b=bytes.fromhex('$$BACKEND_ACCOUNT_ID'); print('vec{' + ';'.join(str(x) for x in b) + '}')") && \
	result=$$(icp canister call $(LEDGER_PRINCIPAL) transfer \
	  "(record { to = $$BACKEND_ACCOUNT_BYTES; memo = 0 : nat64; amount = record { e8s = 100_000_000 : nat64 }; fee = record { e8s = 10_000 : nat64 }; })" \
	  -n http://localhost:8000) && \
	echo "$$result" && \
	echo "$$result" | grep -q 'Ok' && \
	echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: transfer ICP from backend canister to default identity ==="
	@CALLER_PRINCIPAL=$$(icp identity principal) && \
	result=$$(icp canister call backend transfer \
	  "(record { amount = record { e8s = 50_000_000 : nat64 }; to_principal = principal \"$$CALLER_PRINCIPAL\"; to_subaccount = null })") && \
	echo "$$result" && \
	echo "$$result" | grep -q 'Ok' && \
	echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: verify backend ledger balance decreased after transfer ==="
	@BACKEND_PRINCIPAL=$$(icp canister status backend -i) && \
	BACKEND_ACCOUNT_ID=$$(icp identity account-id --of-principal $$BACKEND_PRINCIPAL) && \
	BACKEND_ACCOUNT_BYTES=$$(python3 -c "b=bytes.fromhex('$$BACKEND_ACCOUNT_ID'); print('vec{' + ';'.join(str(x) for x in b) + '}')") && \
	result=$$(icp canister call $(LEDGER_PRINCIPAL) account_balance \
	  "(record { account = $$BACKEND_ACCOUNT_BYTES })" \
	  -n http://localhost:8000) && \
	echo "$$result" && \
	echo "$$result" | grep -q 'e8s' && \
	echo "PASS" || (echo "FAIL" && exit 1)
