# Internet Computer sample applications

Get started building on ICP with the sample applications in this repository. From this repository, you can deploy, download, clone, fork, or share sample projects.

The projects in this repository are not intended to be used as commercial applications and do not provide any explicit or implied support or warranty of any kind.

You can also contribute your own project or suggest updates to published projects using the standard GitHub workflow.

## Sample applications

Code samples are organized by programming language:

- [Motoko](https://github.com/dfinity/examples/tree/master/motoko)
- [Rust](https://github.com/dfinity/examples/tree/master/rust)
- [C](https://github.com/dfinity/examples/tree/master/c)

Some examples include frontends written in a variety of frameworks such as React, JavaScript, etc.

Additional frontend samples can be found in the following folders:

- [Svelte](https://github.com/dfinity/examples/tree/master/svelte)
- [HTML](https://github.com/dfinity/examples/tree/master/hosting)
- [Unity](https://github.com/dfinity/examples/tree/master/native-apps)

## Try in browser

Many examples include a GitHub Codespaces badge in their README. Clicking it opens a pre-configured environment with the ICP toolchain installed — the local network starts and canisters are deployed automatically. No local setup required.

Browse all your Codespaces at [github.com/codespaces](https://github.com/codespaces).

## Local development

### Dev Containers

Open the repo root in [VS Code](https://code.visualstudio.com/) with the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) to get a pre-configured environment with the full ICP toolchain for both Motoko and Rust. VS Code will prompt you to reopen in the container automatically.

```bash
git clone https://github.com/dfinity/examples.git
```

Then navigate into an example and follow its README to deploy.

> **Note:** The per-example devcontainer configs are designed for GitHub Codespaces. For local Dev Container use, always open the repo root.

### Command line

Install [icp-cli](https://cli.internetcomputer.org), clone the repo, navigate into an example, and follow its README:

```bash
git clone https://github.com/dfinity/examples.git
cd examples/<language>/<example>
```

### ICP Ninja

You can also open and deploy examples with [ICP Ninja](https://icp.ninja/), a web-based tool that requires no local setup. To contribute an example to ICP Ninja, see [NINJA_CONTRIBUTING.md](./NINJA_CONTRIBUTING.md).

## Resources

- [Quickstart](https://docs.internetcomputer.org/getting-started/quickstart)
- [Developer tools](https://docs.internetcomputer.org/developer-tools)
- [icp-cli](https://cli.internetcomputer.org)

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/building-apps/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
