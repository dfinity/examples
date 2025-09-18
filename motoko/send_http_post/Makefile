.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: deploy
.SILENT: deploy
build: node_modules
	dfx canister deploy send_http_post_backend

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call send_http_post_backend send_http_post_request \
		| grep 'Hello' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx