# ICP Codespace

The network starts and canisters are deployed automatically when this Codespace opens. Access URLs are printed in the terminal once deployment completes.

## Access URLs

`icp deploy` prints URLs using `localhost:8000`, which do not work inside a Codespace. Run the following command to get the correct forwarded URLs:

```bash
bash /workspaces/examples/.devcontainer/scripts/show-urls.sh
```

> **Note for non-SPA frontends:** Frontend URLs use a `?canisterId=` query parameter for routing. This works correctly for single-page apps (all navigation stays client-side). If your frontend uses real path-based navigation where clicking a link triggers a new browser request (e.g. navigating to `/page2`), the query parameter will be dropped and the gateway will not know which canister to serve. Subdomain-based routing is not available in Codespaces because GitHub's TLS certificate only covers one subdomain level, making `<canisterId>.<codespace>-8000.app.github.dev` invalid.

## Redeploy

Rebuilds and redeploys all canisters, preserving their state.

```bash
icp deploy
```

## Reset & Redeploy

Reinstalls all canisters from scratch, wiping their state. The network keeps running.

```bash
icp deploy --mode reinstall -y
```
