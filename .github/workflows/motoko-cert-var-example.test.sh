#!/bin/bash
dfx start --background
pushd motoko/cert-var
make test
popd