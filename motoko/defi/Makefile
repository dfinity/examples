t.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	pushd src/frontend
	npm install
	popd

.PHONY: build
.SILENT: build
build: node_modules
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	# XXX Should our script go here?
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	# XXX Should our script go here?
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	# TODO
	# dfx canister call hello greet '("everyone")' \
	# 	| grep '("Hello, everyone!")' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr src/frontend/node_modules/
	rm -rf src/frontend/public/build/
