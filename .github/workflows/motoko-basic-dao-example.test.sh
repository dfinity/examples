#!/bin/bash
dfx start --background
pushd motoko/basic_dao
dfx canister create basic_dao
dfx build
(for f in tests/*.test.sh; do
echo "==== Run test $f ===="
ic-repl -r "http://localhost:$(dfx info webserver-port)" "$f" || exit
done)
popd