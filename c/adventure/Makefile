.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
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

# [TODO] - Provide test
.PHONY: test
.SILENT: test
test: install
	dfx canister call adventure play \
		| grep '("")' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx build