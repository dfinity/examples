.PHONY: all
all: build

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy

.PHONY: mock
.SILENT: mock
mock: deploy
	SCHNORR_MOCK_CANISTER_ID=$(shell dfx canister id schnorr_canister); \
	SCHNORR_EXAMPLE_CANISTER_ID=$(shell dfx canister id schnorr_example_rust); \
	echo "Changing to using mock canister instead of management canister for signing"; \
	CMD="dfx canister call "$${SCHNORR_EXAMPLE_CANISTER_ID}" for_test_only_change_management_canister_id '("\"$${SCHNORR_MOCK_CANISTER_ID}\"")'"; \
	eval "$${CMD}"

.PHONY: test
.SILENT: test
test: mock
	cargo test

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean