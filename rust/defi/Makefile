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

.PHONY: deploy
.SILENT: deploy
deploy:
	$(eval ledger_canister_id := $(shell dfx canister id ledger))
	dfx deploy defi --argument '(record { withdrawl_fee = 0.1 : float32;  ledger_canister_id = principal "$(ledger_canister_id)" } )'

.PHONY: test
.SILENT: test
test: deploy
	$(eval defi_canister_id := $(shell dfx canister id defi))
	dfx canister call defi get_balance '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai")'
	dfx canister call defi deposit '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 100: nat)'
	dfx canister call defi get_order '(1 : nat64)'
	dfx canister call defi place_order '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 1: nat, principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 2: nat)'
	dfx canister call defi get_order '(1 : nat64)'      
	dfx canister call defi get_orders

.PHONY: candid
.SILENT: candid
candid: deploy
	$(eval defi_canister_id := $(shell dfx canister id defi))
	dfx canister call defi __get_candid_interface_tmp_hack 


.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
