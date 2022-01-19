.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	cd src/frontend && npm i
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx deploy 

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	cd src/frontend && npm run test
	cd src/frontend && node scripts/setupTypeScript.js && npm i
	cd src/frontend && npm run build && npm run test

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
