# Continue building locally

Projects deployed through ICP Ninja are temporary; they will only be live for 20 minutes before they are removed. The command-line tool `dfx` can be used to continue building your ICP Ninja project locally and deploy it to the mainnet.

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps.

### 1. Install developer tools.

You can install the developer tools natively or use Dev Containers.

#### Option 1: Natively install developer tools

> Installing `dfx` natively is currently only supported on macOS and Linux systems. On Windows, it is recommended to use the Dev Containers option.

1. Install `dfx` with the following command:

```

sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

```

> On Apple Silicon (e.g., Apple M1 chip), make sure you have Rosetta installed (`softwareupdate --install-rosetta`).

2. [Install NodeJS](https://nodejs.org/en/download/package-manager).

3. For Rust projects, you will also need to:

- Install [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo): `curl https://sh.rustup.rs -sSf | sh`

- Install [candid-extractor](https://crates.io/crates/candid-extractor): `cargo install candid-extractor`

4. For Motoko projects, you will also need to:

- Install the Motoko package manager [Mops](https://docs.mops.one/quick-start#2-install-mops-cli): `npm i -g ic-mops`

Lastly, navigate into your project's directory that you downloaded from ICP Ninja.

#### Option 2: Dev Containers

Continue building your projects locally by installing the [Dev Container extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) for VS Code and [Docker](https://docs.docker.com/engine/install/).

Make sure Docker is running, then navigate into your project's directory that you downloaded from ICP Ninja and start the Dev Container by selecting `Dev-Containers: Reopen in Container` in VS Code's command palette (F1 or Ctrl/Cmd+Shift+P).

> Note that local development ports (e.g. the ports used by `dfx` or `vite`) are forwarded from the Dev Container to your local machine. In the VS code terminal, use Ctrl/Cmd+Click on the displayed local URLs to open them in your browser. To view the current port mappings, click the "Ports" tab in the VS Code terminal window.

### 2. Start the local development environment.

```
dfx start --background
```

### 3. Create a local developer identity.

To manage your project's canisters, it is recommended that you create a local [developer identity](https://internetcomputer.org/docs/building-apps/getting-started/identities) rather than use the `dfx` default identity that is not stored securely.

To create a new identity, run the commands:

```

dfx identity new IDENTITY_NAME

dfx identity use IDENTITY_NAME

```

Replace `IDENTITY_NAME` with your preferred identity name. The first command `dfx start --background` starts the local `dfx` processes, then `dfx identity new` will create a new identity and return your identity's seed phase. Be sure to save this in a safe, secure location.

The third command `dfx identity use` will tell `dfx` to use your new identity as the active identity. Any canister smart contracts created after running `dfx identity use` will be owned and controlled by the active identity.

Your identity will have a principal ID associated with it. Principal IDs are used to identify different entities on ICP, such as users and canisters.

[Learn more about ICP developer identities](https://internetcomputer.org/docs/building-apps/getting-started/identities).

### 4. Deploy the project locally.

Deploy your project to your local developer environment with:

```
npm install
dfx deploy

```

Your project will be hosted on your local machine. The local canister URLs for your project will be shown in the terminal window as output of the `dfx deploy` command. You can open these URLs in your web browser to view the local instance of your project.

### 5. Obtain cycles.

To deploy your project to the mainnet for long-term public accessibility, first you will need [cycles](https://internetcomputer.org/docs/building-apps/getting-started/tokens-and-cycles). Cycles are used to pay for the resources your project uses on the mainnet, such as storage and compute.

> This cost model is known as ICP's [reverse gas model](https://internetcomputer.org/docs/building-apps/essentials/gas-cost), where developers pay for their project's gas fees rather than users pay for their own gas fees. This model provides an enhanced end user experience since they do not need to hold tokens or sign transactions when using a dapp deployed on ICP.

> Learn how much a project may cost by using the [pricing calculator](https://internetcomputer.org/docs/building-apps/essentials/cost-estimations-and-examples).

Cycles can be obtained through [converting ICP tokens into cycles using `dfx`](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/dfx-cycles#dfx-cycles-convert).

### 6. Deploy to the mainnet.

Once you have cycles, run the command:

```

dfx deploy --network ic

```

After your project has been deployed to the mainnet, it will continuously require cycles to pay for the resources it uses. You will need to [top up](https://internetcomputer.org/docs/building-apps/canister-management/topping-up) your project's canisters or set up automatic cycles management through a service such as [CycleOps](https://cycleops.dev/).

> If your project's canisters run out of cycles, they will be removed from the network.

## Additional examples

Additional code examples and sample applications can be found in the [DFINITY examples repo](https://github.com/dfinity/examples).
