NAME = phone_book

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
	dfx canister call phone_book insert '("Police", record { desc = "Call if you have an emergency."; phone = "911" })'
	dfx canister call phone_book lookup '("Police")' \
		| grep '911' && echo 'PASS'
	# This test needs to be rewritten for the new HTTP based architecture
	# dfx canister call www retrieve '("index.js")' \
	#	| grep 'Phone Book' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
