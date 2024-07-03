.PHONY: all
all: build

.PHONY: download_model
.SILENT: download_model
download_model:
	bash ./download_model.sh

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: build
.SILENT: build
build: node_modules download_model
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx deploy --yes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call backend run | grep -w 'tractor' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
