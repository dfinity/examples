#!/bin/bash
dfx start --background
pushd motoko/basic_bitcoin
dfx deploy basic_bitcoin --argument '(variant { regtest })'
popd