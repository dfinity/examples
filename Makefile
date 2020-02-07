# File       : Makefile
# Copyright  : 2020 DFINITY Stiftung
# License    : Apache 2.0 with LLVM Exception
# Maintainer : Enzo Haussecker <enzo@dfinity.org>
# Stability  : Stable

SHELL = /bin/bash

.PHONY: all
all: c motoko

# C.
.ONESHELL: c
.PHONY: c
c:
	cd c
	for dir in $$(ls -d */)
	do
		pushd $$dir
		bash build.sh
		popd
	done

# Motoko.
.ONESHELL: motoko
.PHONY: motoko
motoko:
	cd motoko
	for dir in $$(ls -d */)
	do
		pushd $$dir
		make
		popd
	done
