# Contributing a project to ICP Ninja

We recommend to build your example directly within ICP Ninja, such that it starts out with the correct tooling, structure and configs.
If you do that, your project will naturally be in the correct format and can be easily added to ICP Ninja.
Once the example is done, you can download the source files from ICP Ninja.
Alternatively, you can start off an existing Ninja project from this repo (see CODEOWNERS file for Ninja examples) and modify it.

Ideally, your project should have a frontend and backend.

1) Place your example in either
* `/motoko` for a Motoko project
* `/rust` for a Rust project
* `/hosting` for a frontend-only project

2) Compilation requirements
* make sure the project compiles and runs with `dfx deploy` inside the latest `ghcr.io/dfinity/icp-dev-env-slim` container
* make sure `npm run dev` works and the canister can be called through the browser (if applicable)
* make sure II login works (if applicable)
* if you use Rust, make sure the project has `ic_cdk::export_candid!();` in the lib.rs file, such that the candid interface can be auto derived
* if you use Motoko, use mops as a package manager

3) Frontend tooling requirements
* use React
* use vite for the build system
* use tailwind for styling (recommended), or plain CSS

4) Preparing the PR
* add your project in the CODEOWNERS file, with your team plus `@dfinity/ninja-devs` as reviewers
* add your project to the matrix in `.github/workflows/ninja_pr_checks.yml` to run PR tests
* Add a README.md file, copy the BUILD.md and devcontainer.json file

5) PR to ICP Ninja
* add your newly added project to `frontend/public/projects.json`
* bump the commit hash in `submodules/examples` to a commit hash after you merged into the examples repo

## Templates

### Recommended dfx.json for a Rust canister:

```json
{
  "canisters": {
    "backend": {
      "candid": "backend/backend.did",
      "type": "custom",
      "shrink": true,
      "gzip": true,
      "wasm": "target/wasm32-unknown-unknown/release/backend.wasm",
      "build": [
        "cargo build --target wasm32-unknown-unknown --release -p backend",
        "candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did"
      ],
      "metadata": [
        {
          "name": "candid:service"
        }
      ]
    }
  },
  "output_env_file": ".env"
}
```

### Recommended dfx.json for a Motoko canister:

```json
{
  "canisters": {
    "backend": {
      "main": "backend/app.mo",
      "type": "motoko",
      "args": "--enhanced-orthogonal-persistence"
    },
  },
  "output_env_file": ".env",
  "defaults": {
    "build": {
      "packtool": "mops sources"
    }
  }
}

```

### Recommended dfx.json for a frontend/asset canister:

```json
{
  "canisters": {
    "frontend": {
      "dependencies": ["backend"],
      "frontend": {
        "entrypoint": "frontend/index.html"
      },
      "source": ["frontend/dist"],
      "type": "assets"
    }
  },
  "output_env_file": ".env"
}
```

### Recommended vite.config.js

```js
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import { fileURLToPath, URL } from 'url';
import environment from 'vite-plugin-environment';

export default defineConfig({
  base: './',
  plugins: [react(), environment('all', { prefix: 'CANISTER_' }), environment('all', { prefix: 'DFX_' })],
  envDir: '../',
  define: {
    'process.env': process.env
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: 'globalThis'
      }
    }
  },
  resolve: {
    alias: [
      {
        find: 'declarations',
        replacement: fileURLToPath(new URL('../src/declarations', import.meta.url))
      }
    ]
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
        changeOrigin: true
      }
    },
    host: '127.0.0.1'
  }
});
```
