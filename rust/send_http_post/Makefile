.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy send_http_post_backend

.PHONY: test
.SILENT: test
test: deploy
	# echo 'PASS dummy test'
	dfx canister call send_http_post_backend send_http_post_request \
		| grep 'Hello' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx