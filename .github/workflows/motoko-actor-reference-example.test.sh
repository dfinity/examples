#!/bin/bash
dfx start --background
pushd motoko/actor_reference
make test
popd