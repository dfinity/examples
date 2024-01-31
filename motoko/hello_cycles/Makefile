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
	echo "dfx version: $(shell dfx --version)"
	echo "wallet: $(WALLET)"
	echo "hello_cycles: $(HELLO_CYCLES)"
	echo "$(dfx canister call hello_cycles wallet_balance)"
	# canister has just above 3T cycles, so output start with '(3'
	dfx canister call hello_cycles wallet_balance \
		| grep '(3' && echo 'PASS'
	dfx canister status hello_cycles
	dfx canister call $(WALLET) wallet_send '(record { canister = principal "$(HELLO_CYCLES)"; amount = (2000000000000:nat64); } )'
	# 2T cycles added, now contains just above 5T
	dfx canister call hello_cycles wallet_balance \
		| grep '(5' && echo 'PASS'
	echo  '(func "$(WALLET)"."wallet_receive", 5000000)'
	dfx canister call hello_cycles transfer '(func "$(WALLET)"."wallet_receive", 5000000)' \
		| grep '0' && echo 'PASS'
	dfx canister call hello_cycles wallet_balance \
	 	| grep '(5' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
