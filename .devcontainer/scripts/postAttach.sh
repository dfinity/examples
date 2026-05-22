#!/bin/bash

code CODESPACE.md

icp network start -d 2>/dev/null || true

until curl -sf http://localhost:8000/api/v2/status >/dev/null 2>&1; do
  sleep 1
done

echo "Deploying canisters..."
icp deploy
echo ""
echo "Access URLs:"
echo ""
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh
