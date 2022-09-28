#!/bin/bash

# Stop local replica
dfx stop

# Remove copied and generated UI assets
rm -rf src/frontend
rm -rf src/declarations
rm rollup.config.js
rm -rf node_modules

# Remove package artifacts
rm package.json
rm package-lock.json

# Remove .dfx folder
rm -rf .dfx
