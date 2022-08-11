SHELL = /bin/bash

.PHONY: all
all: test

.ONESHELL: test
.PHONY: test
.SILENT: test
test:
	for dir in $$(ls -d (motoko|c)/*/); do
		pushd $$dir
		make test
		popd
	done

.ONESHELL: clean
.PHONY: clean
.SILENT: clean
clean:
	for dir in $$(ls -d (motoko|c)/*/); do
		pushd $$dir
		make clean
		popd
	done
