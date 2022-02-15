SHELL = /bin/bash
t.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	pushd src/frontend; npm install; popd

.PHONY: install
.SILENT: install
install: clean
	./scripts/install.sh

.PHONY: frontend
.SILENT: frontend
frontend: node_modules
	cd src/frontend && npm run dev 

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx identity use default
	$(eval ledger_canister_id := $(shell dfx canister id ledger))
	dfx deploy defi_dapp --argument "(principal \"$(ledger_canister_id)\")"

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
	dfx stop
	rm -fr .dfx
	rm -fr src/frontend/node_modules/
	rm -rf src/frontend/public/build/
