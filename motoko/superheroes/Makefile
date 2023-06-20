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
	$(eval ID := $(shell dfx canister call superheroes create '(record {name = "Superman"; superpowers = opt record { 0 = "invulnerability"; 1 = opt record { 0 = "superhuman strength"; 1 = null; }; }; })' | tr -d '()'))
	dfx canister call superheroes read '($(ID))' \
		| grep 'Superman' && echo 'PASS'
	dfx canister call superheroes update '($(ID), record {name = "Superman"; superpowers = opt record { 0 = "invulnerability"; 1 = opt record { 0 = "superhuman strength"; 1 = opt record { 0 = "flight"; 1 = opt record { 0 = "x-ray vision"; 1 = null; }; }; }; }; })' \
		| grep 'true' && echo 'PASS'
	dfx canister call superheroes read '($(ID))' \
		| grep 'x-ray vision' && echo 'PASS'
	dfx canister call superheroes delete '($(ID))' \
		| grep 'true' && echo 'PASS'
	dfx canister call superheroes delete '($(ID))' \
		| grep 'false' && echo 'PASS'
	dfx canister call superheroes read '($(ID))' \
		| grep 'null' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
