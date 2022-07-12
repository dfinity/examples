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

e2e:
	./install-local.sh
	npm ci
	npm test

watch:
	while true; do \
		make $(WATCHMAKE); \
		inotifywait --exclude **/.vessel -qre close_write .; \
	done
