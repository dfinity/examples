.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy send_http_get_backend

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call send_http_get_backend send_http_get_request \
		| grep 'hello-from-icp' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx