.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create echo
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install echo

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install echo --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call echo say '("This is a test.")' \
		| grep '("This is a test.")' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
