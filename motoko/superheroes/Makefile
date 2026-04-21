.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: deploy
.SILENT: deploy
deploy: node_modules
	dfx deploy

.PHONY: test
.SILENT: test
test: deploy
	$(eval ID := $(shell dfx canister call superheroes create '(record {name = "Superman"; superpowers = vec { "invulnerability"; "superhuman strength" }})' | tr -d '()'))
	dfx canister call superheroes read '($(ID))' \
		| grep 'Superman' && echo 'PASS'
	dfx canister call superheroes update '($(ID), record {name = "Superman"; superpowers = vec { "invulnerability"; "superhuman strength"; "flight"; "x-ray vision" }})' \
		| grep 'true' && echo 'PASS'
	dfx canister call superheroes read '($(ID))' \
		| grep 'x-ray vision' && echo 'PASS'
	dfx canister call superheroes delete '($(ID))' \
		| grep 'true' && echo 'PASS'
	dfx canister call superheroes delete '($(ID))' \
		| grep 'false' && echo 'PASS'
	dfx canister call superheroes read '($(ID))' \
		| grep 'null' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
