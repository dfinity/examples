#!/bin/bash
dfx start --background
pushd motoko/counter
make test
popd