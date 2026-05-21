#!/bin/bash

echo "ICP local network is running on port 8000."
echo ""

# Deploy if not already deployed (skipped on reconnects)
if ! icp canister status internet_identity_app_backend -i > /dev/null 2>&1; then
  echo "Deploying canisters (this takes a while on first run — Rust compilation included)..."
  icp deploy
  echo ""
  echo "Deployment complete."
else
  echo "Canisters already deployed."
fi

echo ""
echo "Building access URLs..."

BACKEND_ID=$(icp canister status internet_identity_app_backend -i 2>/dev/null)
FRONTEND_ID=$(icp canister status internet_identity_app_frontend -i 2>/dev/null)
# Capture JSON first to avoid broken-pipe panic in the icp-cli binary
NETWORK_JSON=$(icp network status --json 2>/dev/null)
CANDID_UI_ID=$(printf '%s' "$NETWORK_JSON" | jq -r '.candid_ui_principal // empty' 2>/dev/null)
# Fallback: some icp-cli versions expose the Candid UI as a named canister
if [ -z "$CANDID_UI_ID" ]; then
  CANDID_UI_ID=$(icp canister status __Candid_UI -i 2>/dev/null || true)
fi

if [ -n "$CODESPACE_NAME" ]; then
  BASE="https://${CODESPACE_NAME}-8000.app.github.dev"
else
  BASE="http://localhost:8000"
fi

FRONTEND_URL="${BASE}/?canisterId=${FRONTEND_ID}"
CANDID_URL="${BASE}/?canisterId=${CANDID_UI_ID}&id=${BACKEND_ID}"

echo ""
echo "  Frontend:   $FRONTEND_URL"
echo "  Candid UI:  $CANDID_URL"
echo ""
code README.md
