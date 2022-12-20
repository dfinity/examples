.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create http_counter
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install http_counter --mode reinstall --yes

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install http_counter --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	sh -c 'export CANISTER_ID=$$(dfx canister id http_counter); \
	export WEBSERVER_PORT=$$(dfx info webserver-port); \
	(curl -s "$$CANISTER_ID.localhost:$$WEBSERVER_PORT/" --resolve "$$CANISTER_ID.localhost:$$WEBSERVER_PORT:127.0.0.1" | grep "Counter is 0") && \
	(curl -s --compressed "$$CANISTER_ID.localhost:$$WEBSERVER_PORT/" --resolve "$$CANISTER_ID.localhost:$$WEBSERVER_PORT:127.0.0.1" | grep "query") && echo "PASS"'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
