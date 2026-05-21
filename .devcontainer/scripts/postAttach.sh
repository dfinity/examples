#!/bin/bash

echo "ICP local network is running on port 8000."
echo ""

CANISTER_JSON=$(icp canister status --json 2>/dev/null)

if [ -z "$CANISTER_JSON" ]; then
  echo "Deploying canisters..."
  icp deploy
  echo ""
  echo "Deployment complete."
  CANISTER_JSON=$(icp canister status --json 2>/dev/null)
else
  echo "Canisters already deployed."
fi

echo ""
echo "Access URLs:"
echo ""

NETWORK_JSON=$(icp network status --json 2>/dev/null)
CANDID_UI_ID=$(echo "$NETWORK_JSON" | jq -r '.candid_ui_principal // ""')

if [ -n "$CODESPACE_NAME" ]; then
  BASE="https://${CODESPACE_NAME}-8000.app.github.dev"
else
  BASE="http://localhost:8000"
fi

# Detect frontend canisters from `icp project show` (recipes fully expanded).
# A canister is a frontend if its registry_recipe is the asset-canister, or it
# has a sync step with type: assets. Everything else gets a Candid UI link.
FRONTEND_NAMES=$(icp project show 2>/dev/null | node -e '
  let data = "";
  process.stdin.on("data", c => data += c);
  process.stdin.on("end", () => {
    const lines = data.split("\n");
    const frontends = [];
    let inCanisters = false, currentCanister = null;
    for (const line of lines) {
      if (/^\S/.test(line)) {
        inCanisters = (line === "canisters:");
        currentCanister = null;
        continue;
      }
      if (!inCanisters) continue;
      const m = line.match(/^  ([a-z][a-z0-9_-]*):\s*$/);
      if (m) { currentCanister = m[1]; continue; }
      if (currentCanister && !frontends.includes(currentCanister)) {
        if (line.includes("asset-canister") || /^\s+type:\s+assets\s*$/.test(line)) {
          frontends.push(currentCanister);
        }
      }
    }
    process.stdout.write(frontends.join("\n") + (frontends.length ? "\n" : ""));
  });
')

echo "$CANISTER_JSON" | while IFS= read -r entry; do
  ID=$(echo "$entry" | jq -r '.id')
  NAME=$(echo "$entry" | jq -r '.name')
  if echo "$FRONTEND_NAMES" | grep -qx "$NAME"; then
    echo "  $NAME:  ${BASE}/?canisterId=${ID}"
  elif [ -n "$CANDID_UI_ID" ]; then
    echo "  $NAME (Candid UI):  ${BASE}/?canisterId=${CANDID_UI_ID}&id=${ID}"
  fi
done

echo ""
code CODESPACE.md
