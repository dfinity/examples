# Internet Computer sample applications

Get started building on ICP with the sample applications in this repository. From this repository, you can deploy, download, clone, fork, or share sample projects.

> Looking to start a new project from scratch? Use [icp-cli-templates](https://github.com/dfinity/icp-cli-templates) to scaffold a project with `icp new`.

The projects in this repository are not intended to be used as commercial applications and do not provide any explicit or implied support or warranty of any kind.

## Sample applications

**New here?** Start with `hello_world` ([Motoko](motoko/hello_world/) or [Rust](rust/hello_world/)) — a simple full-stack canister with a frontend. Then move on to `who_am_i` ([Motoko](motoko/who_am_i/) or [Rust](rust/who_am_i/)) to add user authentication with [Internet Identity](https://docs.internetcomputer.org/guides/authentication/internet-identity).

Canister examples are organized by language:

- [motoko/](motoko/)
- [rust/](rust/)

Frontend-only and native app examples:

- [hosting/](hosting/) — frontend examples (React, static sites, etc.)
- [native-apps/](native-apps/) — Unity and other native app integrations
- [wasm/](wasm/) — canisters written directly in WebAssembly

## Local development

### Command line

Install the prerequisites and then follow the example's README to deploy.

**Prerequisites:**
- [Node.js](https://nodejs.org/en/download/) (LTS)
- [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- **Motoko examples:** [mops](https://mops.one/docs/install): `npm install -g ic-mops`
- **Rust examples:** [Rust](https://rustup.rs/) + `rustup target add wasm32-unknown-unknown`

See the [full installation guide](https://cli.internetcomputer.org/0.2/guides/installation.md) for platform-specific instructions.

```bash
git clone https://github.com/dfinity/examples.git
cd examples/<language>/<example>
```

### Dev Containers

Open the repo root in [VS Code](https://code.visualstudio.com/) with the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) to get a pre-configured environment with the full ICP toolchain for both Motoko and Rust. VS Code will prompt you to reopen in the container automatically.

```bash
git clone https://github.com/dfinity/examples.git
```

Then navigate into an example and follow its README to deploy.

> **Note:** Open the repo root in the container — not an individual example subfolder.

## Working with AI coding agents

These examples are built to work well with AI coding agents, backed by [ICP skills](https://docs.internetcomputer.org/guides/ai-coding-agents) — tested, frequently-updated instructions that keep an agent aligned with the fast-moving platform.

The repo is set up with **autosync**: opening it in Claude Code runs a `SessionStart` hook that keeps the skills current automatically (you'll be asked to trust the hook the first time). This targets Claude Code today; other agents fetch the skills on demand instead. Details are in [AGENTS.md](AGENTS.md).

## Resources

- [Quickstart](https://docs.internetcomputer.org/getting-started/quickstart)
- [Developer tools](https://docs.internetcomputer.org/developer-tools)
- [icp-cli](https://cli.internetcomputer.org)

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
