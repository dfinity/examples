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

.PHONY: test
.SILENT: test
test: install
	dfx canister call sub init '("Apples")'
	dfx canister call sub getCount \
		| grep '(0 : nat)' && echo 'PASS'
	dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
	sleep 2 # Wait for update.
	dfx canister call sub getCount \
		| grep '(2 : nat)' && echo 'PASS'
	dfx canister call pub publish '(record { "topic" = "Bananas"; "value" = 3 })'
	sleep 2 # Wait for update.
	dfx canister call sub getCount \
		| grep '(2 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
