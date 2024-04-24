#!/bin/bash
dfx start --background
pushd hosting/static-website
dfx deploy
popd