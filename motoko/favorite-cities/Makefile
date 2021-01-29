NAME = favorite_cities

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
	dfx canister call $(NAME) hello_pretty '(vec {"Palo Alto"; "San Francisco"; "Zurich"})' \
		| grep '("Hello from Palo Alto, San Francisco, and Zurich!")' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
