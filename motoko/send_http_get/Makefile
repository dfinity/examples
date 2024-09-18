.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx deploy send_http_get_backend

.PHONY: test
.SILENT: test
test: build
	dfx canister call send_http_get_backend get_icp_usd_exchange \
		| grep '\[1682978460,5\.714,5\.718,5\.714,5\.714,243\.5678\]' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
