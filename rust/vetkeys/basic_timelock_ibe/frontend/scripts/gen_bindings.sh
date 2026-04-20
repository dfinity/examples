#!/bin/bash

cd ../../backend && make extract-candid && dfx generate basic_timelock_ibe && cd ../frontend && rm -r ./src/declarations >> /dev/null 2>&1
mv ../src/declarations ./src && rmdir ../src

# dfx 0.31+ generates @icp-sdk/core imports; rewrite to @dfinity/* to match deps
find ./src/declarations -type f \( -name '*.ts' -o -name '*.js' \) -exec \
  perl -i -pe 's|\@icp-sdk/core/agent|\@dfinity/agent|g; s|\@icp-sdk/core/principal|\@dfinity/principal|g; s|\@icp-sdk/core/candid|\@dfinity/candid|g' {} +