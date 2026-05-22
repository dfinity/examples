#!/bin/bash

code CODESPACE.md

echo "Deploying canisters..."
icp deploy
echo ""
echo "Access URLs:"
echo ""
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh
