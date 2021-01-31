.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create quicksort
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install quicksort

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install quicksort --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call quicksort sort '(vec { 5; 3; 0; 9; 8; 2; 1; 4; 7; 6 })' \
		| grep '(vec { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9 })' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
