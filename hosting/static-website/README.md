# Static website

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/static-website)

## Overview

The example shows how to deploy a simple, static website hosted on ICP. The website is very simple; it just displays the DFINITY logo. While the website in this example is very simple, the method would be the same for a more advanced static website, e.g., based on popular static site generators.

![Website](README_images/website.png)

## Project structure

The website consists of an HTML file, a CSS file, and a PNG file:

```
static-website
├── icp.yaml
└── frontend
    ├── assets
    │   ├── logo.png
    │   └── main.css
    └── src
        └── index.html
```

The `icp.yaml` file is a configuration file that specifies the canister used for the dapp. In this case only one canister is needed.

```yaml
canisters:
  - name: frontend
    recipe:
      type: "@dfinity/asset-canister@v2.1.0"
      configuration:
        dir: dist
        build:
          - mkdir -p dist
          - cp -r frontend/assets/* dist/
          - cp -r frontend/src/* dist/
```

## Prerequisites

- [x] Install [icp-cli](https://cli.icp.build): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

## Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/static-website
```

## Deployment

Start the local network:

```bash
icp network start -d
```

Deploy the canisters:

```bash
icp deploy
```

The URL for the frontend depends on the canister ID. When deployed, the URL will look like this:

```
http://{canister_id}.localhost:8000
```

Stop the local network when done:

```bash
icp network stop
```
