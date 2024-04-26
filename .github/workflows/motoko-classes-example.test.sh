#!/bin/bash
dfx start --background
pushd motoko/classes
make test
popd