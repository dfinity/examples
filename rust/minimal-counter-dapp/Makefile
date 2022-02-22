NAME = minimal_rust_dapp

.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: build
.SILENT: build
build: node_modules
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

.PHONY: test
.SILENT: test
test: install
	dfx canister call minimal_rust_dapp increment \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call minimal_rust_dapp get \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call minimal_rust_dapp reset \
		| grep '(0 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
