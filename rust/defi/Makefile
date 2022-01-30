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
	dfx deploy defi_dapp --argument '(record { withdrawl_fee = 0.1 : float32;  ledger_canister_id = principal "$(ledger_canister_id)" } )'

.PHONY: test
.SILENT: test
test: deploy
	dfx identity use default
	$(eval wallet := $(shell dfx identity get-wallet))
	dfx canister --wallet "$(wallet)" call defi_dapp clear
	-dfx identity new test1
	-dfx identity new test2
	$(eval test1 := $(shell dfx identity use test1 >> /dev/null; dfx identity get-principal))
	$(eval test2 := $(shell dfx identity use test2 >> /dev/null; dfx identity get-principal))
	dfx identity use test1
	dfx identity get-principal
	echo $(principal)
	dfx canister call defi_dapp getBalance '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai")'
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test1)", principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 1: nat)'
	dfx identity use test1
	dfx canister call defi_dapp getBalance '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai")'
	dfx identity use test2
	dfx identity get-principal
	dfx canister call defi_dapp getBalance '(principal "tlwi3-3aaaa-aaaaa-aaapq-cai")'
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test2)", principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 100: nat)'
	dfx identity use test2
	dfx canister call defi_dapp getBalance '(principal "tlwi3-3aaaa-aaaaa-aaapq-cai")'
	dfx identity use test1
	dfx canister call defi_dapp placeOrder '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 1: nat, principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 2: nat)'
	dfx canister call defi_dapp getOrders
	dfx identity use test2
	dfx canister call defi_dapp placeOrder '(principal "tlwi3-3aaaa-aaaaa-aaapq-cai" : principal, 4: nat, principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 2: nat)'
	dfx canister call defi_dapp getOrders
	dfx identity use test1
	dfx canister call defi_dapp placeOrder '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 1: nat, principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 2: nat)'
	dfx canister call defi_dapp getOrders
	dfx canister call defi_dapp getAllBalances
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test1)", principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 1: nat)'
	dfx identity use test1
	dfx canister call defi_dapp getAllBalances
	dfx canister call defi_dapp placeOrder '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 1: nat, principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 2: nat)'
	echo "expect empty vec"
	dfx canister call defi_dapp getOrders
	echo "expect 4 tlwi3"
	dfx canister call defi_dapp getAllBalances
	dfx identity use test2
	echo "expect 96 tlwi3 and 2 tfuft"
	dfx canister call defi_dapp getAllBalances
	dfx identity use default
	dfx canister call defi_dapp getAllBalances
	echo "testing imbalanced trades"
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp clear
	dfx identity use test1
	dfx identity get-principal
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test1)", principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 9: nat)'
	dfx identity use test1
	dfx identity use test2
	dfx identity get-principal
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test2)", principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 2: nat)'
	dfx identity use test2
	dfx canister call defi_dapp getAllBalances
	dfx canister call defi_dapp placeOrder '(principal "tlwi3-3aaaa-aaaaa-aaapq-cai" : principal, 2: nat, principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 1: nat)'
	dfx canister call defi_dapp getOrders
	dfx identity use test1
	dfx canister call defi_dapp placeOrder '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 9: nat, principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 3: nat)'
	dfx canister call defi_dapp getOrders
	dfx canister call defi_dapp getAllBalances
	echo "testing imbalanced trades with non-natural number trade amounts"
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp clear
	dfx identity use test1
	dfx identity get-principal
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test1)", principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 9: nat)'
	dfx identity use test1
	dfx identity use test2
	dfx identity get-principal
	dfx identity use default
	dfx canister --wallet "$(wallet)" call defi_dapp credit '(principal "$(test2)", principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 2: nat)'
	dfx identity use test2
	dfx canister call defi_dapp getAllBalances
	dfx canister call defi_dapp placeOrder '(principal "tlwi3-3aaaa-aaaaa-aaapq-cai" : principal, 2: nat, principal "tfuft-aqaaa-aaaaa-aaaoq-cai", 1: nat)'
	dfx canister call defi_dapp getOrders
	dfx identity use test1
	dfx canister call defi_dapp placeOrder '(principal "tfuft-aqaaa-aaaaa-aaaoq-cai" : principal, 9: nat, principal "tlwi3-3aaaa-aaaaa-aaapq-cai", 4: nat)'
	dfx canister call defi_dapp getOrders
	dfx canister call defi_dapp getAllBalances


.PHONY: candid
.SILENT: candid
candid: deploy
	dfx canister call defi_dapp __get_candid_interface_tmp_hack 

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
