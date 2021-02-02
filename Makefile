SHELL = /bin/bash

.PHONY: all
all: motoko

.ONESHELL: motoko
.PHONY: motoko
.SILENT: motoko
motoko:
	for dir in $$(ls -d motoko/*/); do
		pushd $$dir
		make test
		popd
	done

.ONESHELL: clean
.PHONY: clean
.SILENT: clean
clean:
	for dir in $$(ls -d motoko/*/); do
		pushd $$dir
		make clean
		popd
	done
