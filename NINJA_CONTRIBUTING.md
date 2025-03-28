# Contributing a project to ICP Ninja

We recommend to build your example directly within ICP Ninja, such that it starts out with the correct tooling, structure and configs.
If you do that, your project will naturally be in the correct format and can be easily added to ICP Ninja.
Once the example is done, you can download the source files from ICP Ninja.
Alternatively, you can start with an existing Ninja project from this repo (see CODEOWNERS file for Ninja examples) and modify it.

Ideally, your project should have a frontend and backend.

### Where to place your example
* `/motoko` for a Motoko project
* `/rust` for a Rust project
* `/hosting` for a frontend-only project

### Compilation requirements
* Make sure the project compiles and runs with `dfx deploy` inside the latest `ghcr.io/dfinity/icp-dev-env-slim` container.
* Make sure `npm run dev` works and the canister can be called through the browser (if applicable).
* Make sure II login works (if applicable).
* If you use Rust, make sure the project has `ic_cdk::export_candid!();` in the `lib.rs` file, such that the Candid interface can be auto derived.
* If you use Motoko, use Mops as the package manager.

### Frontend tooling requirements
* React
* Vite for the build system
* Tailwind CSS for styling (recommended), or plain CSS

## Preparing the PR
1. Add your project in the `CODEOWNERS` file, with your team plus `@dfinity/ninja-devs` as reviewers.
2. Add your project to the matrix in `.github/workflows/ninja_pr_checks.yml` to run PR tests.
3. Add a `README.md` file, copy the `BUILD.md` and `devcontainer.json` files.

## Submit a PR to ICP Ninja
1. Add your newly added project to `frontend/public/projects.json`
2. Bump the commit hash in `submodules/examples` to a commit hash after your PR has been merged into the examples repo.
* ask the Ninja team or AI to give you a beautiful image for your project

## Templates

### Recommended `dfx.json` for a Rust canister:

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

### Recommended `dfx.json` for a Motoko canister:

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

### Recommended `dfx.json` for a frontend/asset canister:

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

### Recommended `vite.config.js`

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
