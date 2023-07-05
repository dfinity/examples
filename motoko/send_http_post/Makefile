.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: build
.SILENT: build
build: node_modules
	dfx canister create send_http_post_backend
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install send_http_post_backend --mode reinstall --yes


.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install send_http_post_backend --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call send_http_post_backend send_http_post_request \
		| grep 'success.*true' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx