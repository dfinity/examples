#!/bin/bash

# turn on bash's job control

BUILD_ENV=${BUILD_ENV:-motoko}
BACKEND_HEADER_FILE="src/frontend/src/lib/backend.ts"
IDL_FACTORY_FILE="src/frontend/src/lib/idlFactory.js"

echo "=== Deploying encrypted_notes_${BUILD_ENV} ==="

sed "s/{{ BUILD_ENV }}/${BUILD_ENV}/" dfx.json.template > dfx.json
echo "--- Generated dfx.json ---"

if [ "$BUILD_ENV" = "motoko" ]; then 
read -r -d '' BACKEND_TS <<- EOM
export { idlFactory } from './idlFactory';
export * from '../../../declarations/encrypted_notes_motoko/encrypted_notes_motoko.did.js';
export const ENCRYPTED_NOTES_CANISTER_ID = process.env.ENCRYPTED_NOTES_MOTOKO_CANISTER_ID;
EOM
else 
read -r -d '' BACKEND_TS <<- EOM
export { idlFactory } from './idlFactory';
export type { _SERVICE } from '../../../declarations/encrypted_notes_rust/encrypted_notes_rust.did.js';
export const ENCRYPTED_NOTES_CANISTER_ID = process.env.ENCRYPTED_NOTES_RUST_CANISTER_ID;
EOM
fi
echo "$BACKEND_TS" > "$BACKEND_HEADER_FILE"
echo "--- Generated $BACKEND_HEADER_FILE ---"

if [ "$BUILD_ENV" = "motoko" ]; then 
read -r -d '' IDL_FACTORY_TS <<- EOM
// d.ts file is broken, importing idlFactory doesn't work from a .ts file
export { idlFactory } from '../../../declarations/encrypted_notes_motoko/encrypted_notes_motoko.did.js';
EOM
else 
read -r -d '' IDL_FACTORY_TS <<- EOM
// d.ts file is broken, importing idlFactory doesn't work from a .ts file
export { idlFactory } from '../../../declarations/encrypted_notes_rust/encrypted_notes_rust.did.js';
EOM
fi
echo "$IDL_FACTORY_TS" > "$IDL_FACTORY_FILE"
echo "--- Generated $IDL_FACTORY_FILE ---"