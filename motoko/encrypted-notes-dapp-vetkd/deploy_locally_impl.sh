#!/bin/bash

npm install
rm -fr .dfx
dfx start --clean --background
dfx deploy internet_identity --argument '(null)'
dfx deploy encrypted_notes_${BUILD_ENV}
dfx generate encrypted_notes_${BUILD_ENV}
# export NODE_OPTIONS=--openssl-legacy-provider
dfx deploy www
npm run dev-docker