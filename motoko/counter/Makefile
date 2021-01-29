NAME = counter

.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create $(NAME)
	dfx build $(NAME)

.PHONY: install
.SILENT: install
install: build
	dfx canister install $(NAME)

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --mode=upgrade $(NAME)

.PHONY: test
.SILENT: test
test: install
	dfx canister call $(NAME) set '(7)'
	dfx canister call $(NAME) inc
	dfx canister call $(NAME) get \
		| grep '(8)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
