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
	dfx canister call calc add '(9)' \
		| grep '(9 : int)' && echo 'PASS'
	dfx canister call calc sub '(1)' \
		| grep '(8 : int)' && echo 'PASS'
	dfx canister call calc mul '(3)' \
		| grep '(24 : int)' && echo 'PASS'
	dfx canister call calc div '(6)' \
		| grep '(opt (4 : int))' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
