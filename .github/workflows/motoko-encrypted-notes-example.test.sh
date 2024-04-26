#!/bin/bash
pushd motoko/encrypted-notes-dapp
make test-e2e BUILD_ENV=motoko
popd