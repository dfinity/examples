# Random maze

The example generates a random maze using cryptographic randomness.

It illustrates:

- Importing library `Random` from `mo:core` to use cryptographic randomness.
- Obtaining an `AsyncRandom` instance via `Random.crypto()`, which automatically fetches entropy from the management canister on demand — no manual blob management needed.
- Generating bounded, discrete random numbers using `await* random.natRange(low, high)`.

The application is built from the following Motoko source code file:

- `main.mo`: contains the actor definition and methods exposed by this canister.

This actor uses Motoko's random library to generate a cryptographically random maze of user-specified size.

The function `generate` uses `Random.crypto()` to obtain an `AsyncRandom` instance that transparently fetches entropy from the Internet Computer whenever needed. Bounded random numbers are sampled with `await* random.natRange(0, n)`, which handles entropy replenishment automatically.

This is a Motoko example that does not currently have a Rust variant.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/motoko/random_maze
dfx start --background
```

## Step 2: Install front-end dependencies

```bash
npm install
```

## Step 3: Deploy the canisters

```bash
dfx deploy
```

## Step 4: Take note of the URL at which the user interface is accessible

```bash
echo "http://127.0.0.1:4943/?canisterId=$(dfx canister id random_maze_assets)"
```

Enter a size for the maze, then select **Generate!**. The maze will be displayed.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it employs cryptographic algorithms:
* [Don’t implement crypto yourself.](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#dont-implement-crypto-yourself)
* [Use secure cryptographic schemes.](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#use-secure-cryptographic-schemes)

