.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: mock
.SILENT: mock
mock: install
	SCHNORR_MOCK_CANISTER_ID=$(shell dfx canister id schnorr_canister); \
	SCHNORR_EXAMPLE_CANISTER_ID=$(shell dfx canister id schnorr_example_motoko); \
	echo "Changing to using mock canister instead of management canister for signing"; \
	CMD="dfx canister call "$${SCHNORR_EXAMPLE_CANISTER_ID}" for_test_only_change_management_canister_id '("\"$${SCHNORR_MOCK_CANISTER_ID}\"")'"; \
	eval "$${CMD}"

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx