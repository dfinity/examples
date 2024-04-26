#!/bin/bash
pushd motoko/defi
bash ./scripts/install.sh
bash ./test/demo.sh
bash ./test/trade.sh
bash ./test/transfer.sh
popd