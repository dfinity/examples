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
	dfx canister install quotes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install quotes --mode=upgrade

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy quotes

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call quotes setup | grep -qw '()' && echo 'PASS'
	dfx canister call quotes print | grep -qw 'Colourless green ideas sleep furiously.' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
