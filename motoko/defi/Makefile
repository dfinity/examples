SHELL = /bin/bash
t.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	pushd src/frontend; npm install; popd

.PHONY: build
.SILENT: build
build: node_modules
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	# XXX Should our script go here?
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	# XXX Should our script go here?
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test:
	./test/demo.sh
	./test/trade.sh
	./test/transfer.sh

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr src/frontend/node_modules/
	rm -rf src/frontend/public/build/
