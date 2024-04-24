#!/bin/bash
dfx start --background
pushd hosting/unity-webgl-template
dfx deploy
popd