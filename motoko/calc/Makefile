.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create calc
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install calc

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install calc --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call calc add '(9)'
	dfx canister call calc sub '(1)'
	dfx canister call calc mul '(3)'
	dfx canister call calc div '(6)' \
		| grep '(opt 4)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
