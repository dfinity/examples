#!/bin/bash

code CODESPACE.md

icp network start -d 2>/dev/null || true

echo "Deploying canisters..."
icp deploy
echo ""
echo "Access URLs:"
echo ""
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh
