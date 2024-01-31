.PHONY: all
all: deploy

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: deploy
.SILENT: deploy
deploy: node_modules
	dfx canister create system_api --specified-id s55qq-oqaaa-aaaaa-aaakq-cai
	dfx deploy

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call app_backend symmetric_key_verification_key && echo 'MINIMAL SANITY CHECK PASSED'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean
