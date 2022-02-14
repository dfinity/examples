SHELL = bash -xv

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
	dfx identity use default
	$(eval ledger_canister_id := $(shell dfx canister id ledger))
	dfx deploy defi_dapp --argument "(principal \"$(ledger_canister_id)\")"

.PHONY: test
.SILENT: test
test:
	./test/demo.sh
	./test/trade.sh
	./test/transfer.sh

.PHONY: candid
.SILENT: candid
candid: deploy
	dfx canister call defi_dapp __get_candid_interface_tmp_hack 

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
