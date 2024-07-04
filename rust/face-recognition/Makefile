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
	dfx deploy --yes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call backend run_detection | grep -w 'top = 0.15649638' && echo 'PASS'
	dfx canister call backend run_recognition | grep -w 'Unknown person' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
