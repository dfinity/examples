.PHONY: all
all: test

.PHONY: install
.SILENT: install
install: ./install-local.sh

.PHONY: test
.SILENT: test
test: install
	$(shell vessel bin)/moc -r $(shell vessel sources) -wasi-system-api test/unit/Test.mo

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx

# fetch_ledger


e2e:
	npm ci
	dfx stop
	dfx start --background --clean
	node ./install-local.mjs
	npm test

watch:
	while true; do \
		make $(WATCHMAKE); \
		inotifywait --exclude **/.vessel -qre close_write .; \
	done
