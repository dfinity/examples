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
	dfx canister install internet_identity --argument '(null)'
	dfx canister install greet_backend
	dfx canister install greet_frontend

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install internet_identity --argument '(null)' --mode=upgrade
	dfx canister install greet_backend --mode=upgrade
	dfx canister install greet_frontend --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call greet_backend greet \
		| grep '("Hello,' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
	rm -fr src/declarations
