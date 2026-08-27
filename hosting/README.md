# Hosting examples

Frontend-only examples that run entirely on the Internet Computer — static sites, web apps, and game builds served on-chain with no custom backend. Each example is self-contained: follow its README to deploy it locally with `icp-cli`.

These examples use the `@dfinity/static-site` recipe, which deploys the [certified-assets canister](https://github.com/dfinity/certified-assets) — the successor of the legacy [asset canister](https://docs.internetcomputer.org/guides/frontends/asset-canister) going forward, adding response certification, clean-URL canonicalization, and `_headers`/`_redirects` configuration.

**New to ICP?** Start with [`static-website`](static-website/), the smallest possible deployment, then [`react`](react/) for a typical single-page app setup.

Prerequisites and dev-container setup are covered in the [repository README](../README.md).

## Examples

- [`static-website`](static-website/) — deploy a plain HTML/CSS website on ICP.
- [`react`](react/) — a minimal React single-page application hosted fully on-chain.
- [`godot-html5-template`](godot-html5-template/) — deploy a Godot HTML5 game build.
- [`unity-webgl-template`](unity-webgl-template/) — deploy a Unity WebGL game build.
- [`oisy-signer-demo`](oisy-signer-demo/) — connect to the OISY wallet signer, fetch token balances, and perform ICRC transfers.

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
