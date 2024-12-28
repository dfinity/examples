.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create hello_world
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install hello_world

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install hello_world --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call hello_world main \
		| grep '()' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
