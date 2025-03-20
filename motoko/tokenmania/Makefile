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
	dfx canister call backend create_token '(record { token_name = "name"; token_symbol = "TKN"; initial_supply = 5; token_logo = "logo"; })' \
		| grep '(variant { Ok = "Token created" })' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
