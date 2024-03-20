---
keywords: [beginner, motoko, phone book, directory]
---

# Phone book

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/phone-book)

## Overview

This example demonstrates a phone book application that is accessible from your web browser.

The application is built from the following Motoko source code files:

- `index.jsx`: contains the JavaScript, React, and HTML used to generate the front-end user interface for the application when it is launched in a web browser.
- `Main.mo`: contains the actor definition and methods exposed by this canister.

This is a Motoko example that does not currently have a Rust variant. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install [Node.js](https://nodejs.org/en/download/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/motoko/phone-book
dfx start --background
```

### Step 2: Install front-end dependencies:

```bash
npm install
```

### Step 3: Deploy the canister:

```bash
dfx deploy
```

### Step 4: Take note of the URL at which the phone book is accessible.

```bash
echo "http://127.0.0.1:4943/?canisterId=$(dfx canister id www)"
```

### Step 5: Open the aforementioned URL in your web browser.

You will see an interface that you can interact with to store phone book entries.
