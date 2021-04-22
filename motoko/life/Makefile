.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	cp -r versions/v0 src
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
	rm -r -f src
	cp -r versions/v1 src
	dfx deploy \
		| grep 'upgraded to v1' && echo 'PASS'
	rm -r -f src
	cp -r versions/v2 src
	dfx deploy \
		| grep 'upgraded to v2' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr src
	rm -fr .dfx
