SHELL = /bin/bash

.PHONY: all
all: install

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	pushd src/frontend; npm install; popd

.PHONY: install
.SILENT: install
install:
	./scripts/install.sh

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	dfx build

.PHONY: frontend
.SILENT: frontend
frontend: node_modules
	cd src/frontend && npm run dev

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
	dfx stop
	rm -fr .dfx
	rm -fr src/frontend/node_modules/
	rm -fr src/frontend/declarations/
	rm -fr src/frontend/build/
