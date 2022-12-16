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
	dfx canister install --all --mode=auto

.PHONY: test
.SILENT: test
test: install
	sh test_api.sh
	echo "IOS NOTIFICATIONS TESTS PASSED"

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
