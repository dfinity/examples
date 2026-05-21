# ICP Codespace

Canisters are deployed automatically when the Codespace starts.

## Show URLs

```bash {"name":"show-urls","interactive":false}
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh
```

## Redeploy

Rebuilds and redeploys all canisters, preserving their state.

```bash {"name":"redeploy"}
icp deploy
```

## Reset & Redeploy

Reinstalls all canisters from scratch, wiping their state. The network keeps running.

```bash {"name":"reset-and-redeploy"}
icp deploy --mode reinstall -y
```
