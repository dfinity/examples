#!/bin/sh
# Start bitcoind in regtest mode, then hand off to the IC network launcher.
# bitcoind runs in the background; the launcher becomes PID 1 via exec.

bitcoind \
  -regtest -server \
  -rpcbind=0.0.0.0 -rpcallowip=0.0.0.0/0 \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  -fallbackfee=0.00001 -txindex=1 &

# Wait for bitcoind to accept RPC connections
until bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  getblockcount >/dev/null 2>&1; do
  sleep 0.5
done

echo "bitcoind ready on regtest"

# Hand off to the IC network launcher.
# --bitcoind-addr wires the IC Bitcoin subnet to our local bitcoind.
# Port 18443 is the RPC port (used in Makefile via curl JSON-RPC).
# Port 18444 is the P2P port (used by the launcher for block discovery).
exec /app/icp-cli-network-launcher \
  --status-dir=/app/status \
  --config-port 4942 \
  --gateway-port 4943 \
  --bind 0.0.0.0 \
  --pocketic-config-bind 0.0.0.0 \
  --bitcoind-addr=127.0.0.1:18444 \
  "$@"
