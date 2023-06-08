SHELL = /bin/bash

.PHONY: all
all: install

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	pushd src/frontend; npm install; popd

.PHONY: install
.SILENT: install
install: clean
	./scripts/install.sh


.PHONY: init-local
.SILENT: init-local
init-local: 
	./scripts/initalize_local_balance.sh $(II_PRINCIPAL)

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

.PHONY: clean
.SILENT: clean
clean:
	dfx stop
	rm -fr .dfx
	rm -fr src/frontend/node_modules/
	rm -fr src/frontend/declarations/
	rm -fr src/frontend/build/
