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
	$(eval WALLET := $(shell dfx identity get-wallet))
	$(eval HELLO_CYCLES := $(shell dfx canister id hello_cycles))
	echo $(WALLET)
	echo $(HELLO_CYCLES)
	dfx canister call hello_cycles wallet_balance \
		| grep '(4_000_000_000_000 : nat)' && echo 'PASS'
	dfx canister status hello_cycles
	dfx canister call $(WALLET) wallet_send '(record { canister = principal "$(HELLO_CYCLES)"; amount = (2000000000000:nat64); } )'
	dfx canister call hello_cycles wallet_balance \
		| grep '(4_000_010_000_000 : nat)' && echo 'PASS'
	echo  '(func "$(WALLET)"."wallet_receive", 5000000)'
	dfx canister call hello_cycles transfer '(func "$(WALLET)"."wallet_receive", 5000000)' \
		| grep '0' && echo 'PASS'
	dfx canister call hello_cycles wallet_balance \
	 	| grep '(4_000_005_000_000 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
