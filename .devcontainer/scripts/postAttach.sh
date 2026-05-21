#!/bin/bash

echo "ICP local network is running on port 8000."
echo ""

CANISTER_JSON=$(icp canister status --json 2>/dev/null)

if [ -z "$CANISTER_JSON" ]; then
  echo "Deploying canisters..."
  icp deploy
  echo ""
  echo "Deployment complete."
fi

echo ""
echo "Access URLs:"
echo ""
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh

echo ""
code CODESPACE.md
