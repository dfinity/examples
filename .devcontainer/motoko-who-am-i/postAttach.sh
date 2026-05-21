#!/bin/bash

echo "ICP local network is running on port 8000."
echo ""

# Deploy if not already deployed (skipped on reconnects)
if ! icp canister status internet_identity_app_backend -i > /dev/null 2>&1; then
  echo "Deploying canisters (this takes ~30 seconds on first run)..."
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
CANDID_UI_ID=$(icp network status --json | jq -r '.candid_ui_principal' 2>/dev/null)

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
