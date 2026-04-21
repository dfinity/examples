#!/bin/bash

cd ../../backend && make extract-candid

cd .. && dfx generate basic_ibe || exit 1

rm -r frontend/src/declarations/basic_ibe > /dev/null 2>&1 || true

mkdir -p frontend/src/declarations/basic_ibe
mv src/declarations/basic_ibe frontend/src/declarations
rmdir -p src/declarations > /dev/null 2>&1 || true

# dfx 0.31+ generates @icp-sdk/core imports; rewrite to @dfinity/* to match deps
find frontend/src/declarations -type f \( -name '*.ts' -o -name '*.js' \) -exec \
  perl -i -pe 's|\@icp-sdk/core/agent|\@dfinity/agent|g; s|\@icp-sdk/core/principal|\@dfinity/principal|g; s|\@icp-sdk/core/candid|\@dfinity/candid|g' {} +