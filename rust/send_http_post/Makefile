.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy send_http_post_backend

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call send_http_post_backend send_http_post_request \
		| grep 'POST request from an ICP canister' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx