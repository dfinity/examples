#!/bin/bash
dfx start --background
pushd motoko/echo
make test
popd