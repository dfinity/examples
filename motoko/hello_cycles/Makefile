.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create hello_cycles
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install hello_cycles

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install hello_cycles --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call hello_cycles wallet_balance \
		| grep '(10_000_000_000_000)' && echo 'PASS'
	dfx canister status hello_cycles \
		| grep 'Balance: 10_000_000_000_000 Cycles' && echo 'PASS'
	dfx canister call $(dfx identity get-wallet) wallet_send "(record { canister = principal \"$(dfx canister id hello_cycles)\"; amount = (2000000000000:na
t64); } )"
	dfx canister call hello_cycles wallet_balance \
	 	| grep '(10_000_010_000_000)' && echo 'PASS'
	dfx canister call hello_cycles transfer "(func \"$(dfx identity get-wallet)\"	.\"wallet_receive\", 5000000)" \
		| grep '(record { refunded = 0 } )' && echo 'PASS'
	dfx canister call hello_cycles wallet_balance \
	 	| grep '(10_000_005_000_000)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
