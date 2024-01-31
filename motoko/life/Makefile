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
	echo 'v0'
	dfx canister call life current | grep '0' && echo 'PASS'
	echo 'v1'
	rm -r -f src
	cp -r versions/v1 src
	dfx deploy
	dfx canister call life current | grep '1' && echo 'PASS'
	echo 'v2'
	rm -r -f src
	cp -r versions/v2 src
	dfx deploy
	dfx canister call life current | grep '2' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
