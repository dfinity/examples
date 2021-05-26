.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create actor_reference
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install actor_reference

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install actor_reference --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call actor_reference burn '()' \
		| grep '()' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
