t.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create hello
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install hello

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install hello --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call greet "("Mom")' \
		| grep '("Hello, Mom!")' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
