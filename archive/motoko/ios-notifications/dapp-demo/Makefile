.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx start --clean --background
	dfx canister create ios_notifications_api
	dfx build ios_notifications_api

.PHONY: install
.SILENT: install
install: build
	dfx canister install ios_notifications_api --mode=auto

.PHONY: test
.SILENT: test
test: install
	sh test_api.sh
	echo "IOS NOTIFICATIONS TESTS PASSED"

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
