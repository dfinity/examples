#!/bin/bash
dfx start --background
pushd motoko/calc
make test
popd