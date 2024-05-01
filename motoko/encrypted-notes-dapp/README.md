---
keywords: [advanced, motoko, encrypted notes, encrypted, notes dapp]
---

# Encrypted notes

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/encrypted-notes-dapp)

Encrypted notes is an example dapp for authoring and storing confidential information on the Internet Computer (ICP) in the form of short pieces of text. Users can create and access their notes via any number of automatically synchronized devices authenticated via [Internet Identity (II)](https://wiki.internetcomputer.org/wiki/What_is_Internet_Identity). Notes are stored confidentially thanks to the end-to-end encryption performed by the dapp’s frontend.

This project serves as a simple (but not too simple) example of a dapp, which uses Motoko and Rust as backend and Svelte as frontend.

<p align="center">
  <img src="https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/pictures/architectural_overview.png?raw=true" width="70%" height="70%"/>
</p>
<p align = "center">
Fig.1. Architectural overview of the Encrypted Notes dapp using client-side end-to-end encryption.
</p>

---
&nbsp;

## Disclaimer: please read carefully

This is an **example dapp** that demonstrates the potential of building **canisters** for the IC. Please do not use this code in production and/or scenarios in which sensitive data could be involved. While this dapp illustrates end-to-end encryption, there are **several open security issues** that should be addressed before the dapp could be considered production-ready:

- The frontend re-uses the generated public and private key pair for every identity in the same browser. In a better implementation, this key pair should be unique per principal.
- The public/private key pair should not be managed by the web browser at all. [WebAuthn](https://en.wikipedia.org/wiki/WebAuthn) should be used to push the key management to the operating system.
- Integer overflows are possible in the Rust canister, e.g., for `NEXT_NOTE`. 
- Users may lose their notes if they accidentally clean the browser data (localStorage) while no other device is synced to the dapp.
- Lack of key update: Given that the key used to encrypt the notes is never refreshed, the privacy of the data is no longer guaranteed if an attacker learns this key (for instance, by corrupting the local storage in one of the connected devices).

---
&nbsp;

## Overview

You can play around with the [dapp deployed on ICP](https://cvhrw-2yaaa-aaaaj-aaiqa-cai.icp0.io/) and see a quick introduction on [YouTube](https://youtu.be/DZQmtPSxvbs).

We wanted to build an example of a simple (but not too simple) dapp running purely on the IC. This example relies upon the **web-serving** and **storage capabilities** of the IC. We focused on the following two key features for our example dapp: 
1. Client-side **end-to-end encryption**. 
2. **Multi-user** and **multi-device** support.

To demonstrate the potential of the IC as a platform for developing such dapps, we implemented this example using two distinct canister development kits (CDKs). The Motoko CDK allows developers to implement actor-based dapps using the [Motoko](https://internetcomputer.org/docs/current/motoko/getting-started/motoko-introduction) language. The Rust CDK allows implementing dapps in [Rust](https://internetcomputer.org/docs/current/developer-docs/backend/rust/index). In both cases, canisters are compiled into WebAssembly files that are then deployed onto the IC.

## Architecture

The basic functionality of the encrypted notes consists of two main components.

First, we re-used the code of a non-encrypted dapp called [IC Notes](https://github.com/pattad/ic_notes). In particular, IC Notes relies on the Internet Identity (II) canister for user authentication, an approach that is also inherited by the encrypted notes dapp. For development purposes, we deploy a local instance of the II canister, along with a local instance of encrypted notes. When deploying the encrypted notes dapp onto the mainnet, the real-world instance of II is used for authentication.

Second, we enabled client-side, end-to-end encryption for the note contents, borrowing the solution from another existing dapp called [IC Vault](https://github.com/timohanke/icvault). Our encrypted notes dapp follows the approach of IC Vault to support managing multiple devices.

In the context of the canisters discussed in this document, a device is not necessarily a separate physical device but a logical instance device, e.g., a web browser, with its own local data storage. For example, we consider two web browsers running on the same laptop as two independent devices, since these browsers generate their encryption keys. In contrast, the II canister relies on hardware-generated encryption keys, distinguishing only hardware devices.

To support multiple devices per user, IC Vault employs a device manager; a canister that securely synchronizes device-specific keys across all the devices that are associated with a user. The remainder of this document focuses on the encrypted notes dapp canister that similarly implements a device manager but as part of its main canister.

For further details and user stories, please refer to the [README file](https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/README.md).

## Note management

-   Users are linked to II in the frontend, getting the user a principal that can be used for calling API queries and updates.

-   Internally, we store the map of the form `Principal → [Notes]` and a `counter`.

-   `counter` stores the number of notes the canister has created across all principals.

-   Method `create` adds a note to its principal’s entry (if it exists), or adds the principal to the map with the `note_id == counter`, and then increments `counter`.

-   Method `update` pulls a note, for the caller’s principal and the provided `note_id` and replaces it with the provided `text` (this `text` is assumed to be encrypted by the frontend).

-   Method `delete` finds the note with the given `note_id` in the map and removes it. To ensure that note IDs are always globally unique, we do not decrease `counter`.

## Cryptography

Encryption of notes is entirely client-side. However, our example dapp is still not protected against potentially data-revealing attacks by a possibly malicious node provider. For example, the attacker can infer how many notes a particular user has, user activity statistics, etc. Therefore, please carefully read the [disclaimer](https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/README.md#disclaimer-please-read-carefully) before using any of the code or patterns from this dapp.

Recall that, in our definition, a device is not necessarily a separate physical device but simply a web browser instance with independent local storage.

This dapp uses three different kinds of keys:

-   **Symmetric AES-GCM secret key**: Used to encrypt the notes of a given principal. The notes of a principal are stored in the encrypted notes dapp canister encrypted with this secret key. Thus, the frontend of the dapp needs to know this secret key to decrypt notes from this user and to send encrypted notes to be stored in the Encrypted Notes canister.

-   **Device RSA-OAEP public key**: used to encrypt the symmetric AES **secret key** of the principal. The encrypted secret key is stored in the canister for each device registered to the principal. The same key is used for different principals using that device.

-   **Device RSA-OAEP private key**: used to decrypt the symmetric AES **secret key** stored in the encrypted notes canister for a given principal. Once the frontend decrypts the secret key, it can use this key for decrypting the notes stored in the encrypted notes canister.

We store a map of the form:

        Principal → (DeviceAlias → PublicKey,
                     DeviceAlias → CipherText)

This map is used for managing user devices, as explained next. To register a device, the frontend generates a device alias, a public key, and a private key (held in its local storage).

Adding a device:

-   **Device registration:** If this identity is already known, a new device will remain unsynced at first; at this time, only the `alias` and `publickey` of this device will be added to the Encrypted Notes canister.

-   **Device synchronization:** Once an unsynced device obtains the list of all unsynced devices for this II, it will encrypt the symmetric AES **secret key** under each unsynced device’s public key. Afterward, the unsynced device obtains the encrypted symmetric AES **secret key**, decrypts it, and then uses it to decrypt the existing notes stored in the encrypted notes canister.

Once authenticated with II:

-   If this identity is not known, then the frontend generates a symmetric AES **secret key** and encrypts it with its own public key. Then the frontend calls `seed(publickey, ciphertext)`, adding that `ciphertext` and its associated `publickey` to the map.

-   If a user wants to register a subsequent device, the frontend calls `register_device`, passing in the `alias` and `publickey` of that device. The frontend then calls `submit_ciphertexts([publickey, ciphertext])` for all the devices it needs to register. This allows the registered devices to pull and decrypt the AES key to encrypt and decrypt the user notes.

## Encrypted note-taking dapp tutorial

Follow the steps below to deploy this sample project.

## Prerequisites
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index).
- [x] Download and install [Docker](https://docs.docker.com/get-docker/) if using the Docker option. 
- [x] Download the GitHub repo containing this project's files: `git clone https://github.com/dfinity/examples`

### Step 1. Navigate inside of the project's folder:

```bash
cd examples/motoko/encrypted-notes-dapp
```

This project folder contains the files for both Motoko and Rust development.

### Step 2: Set an environmental variable reflecting which backend canister you'll be using:

For Motoko deployment run:

```bash
export BUILD_ENV=motoko
```


**Building the Rust canister requires either the Rust toolchain installed on your system or Docker-backed deployment (see below).**
 
### Step 3: Deploy locally. 

### Option 1: Docker deployment
**This option does not yet work on Apple M1; the combination of DFX and Docker do not currently support the required architecture.**

- #### Step 1: Install and start Docker by following the instructions.
- #### Step 2: For Motoko build/deployment set environmental variable:
        
```bash
export BUILD_ENV=motoko
```

- #### Step 3: Run the following Bash script that builds a Docker image, compiles the canister, and deploys this dapp (all inside the Docker instance). 

Execution can take a few minutes:

```bash
sh ./deploy_locally.sh
```


**If this fails with "No such container", please ensure that the Docker daemon is running on your system.**

- #### Step 4: To open the frontend, go to `http://localhost:3000/`.

- #### Step 5: To stop the docker instance:
   - Hit **Ctrl+C** on your keyboard to abort the running process.
   - Run `docker ps` and find the `<CONTAINER ID>` of encrypted_notes.
   - Run `docker rm -f <CONTAINER ID>`.

### Option 2: Manual deployment
- #### Step 1: For Motoko deployment set environmental variable:

```bash
export BUILD_ENV=motoko
```

- #### Step 2: To generate $BUILD_ENV-specific files (i.e., Motoko or Rust) run:

```bash
sh ./pre_deploy.sh
```

- #### Step 3: Install `npm` packages from the project root:

```bash
npm install
```

- #### Step 4: Start `dfx`:

```bash
dfx start
```


:::info
If you see an error "Failed to set socket of tcp builder to 0.0.0.0:8000", make sure that the port 8000 is not occupied, e.g., by the previously run Docker command (you might want to stop the Docker daemon whatsoever for this step).
:::

- #### Step 5: Install a local Internet Identity (II) canister.

:::info
If you have multiple `dfx` identities set up, ensure you are using the identity you intend to use with the `--identity` flag.
:::

To install and deploy a canister run:

```bash
dfx deploy internet_identity --argument '(null)'
```

- #### Step 6: To print the Internet Identity URL, run:

```bash
npm run print-dfx-ii
```

Visit the URL from above and create at least one local Internet Identity.

- #### Step 7: Deploy the encrypted notes backend canister:

```bash
dfx deploy "encrypted_notes_$BUILD_ENV"
```

**If you are deploying the Rust canister, you should first run `rustup target add wasm32-unknown-unknown`.**

- #### Step 8: Update the generated canister interface bindings:

```bash
dfx generate "encrypted_notes_$BUILD_ENV"
```

- #### Step 9: Deploy the frontend canister.
To install and deploy the canister run:

```bash
dfx deploy www
```

- #### Step 10: To print the frontend canister's URL, run:

```bash
npm run print-dfx-www
```

Visit the URL from above in a web browser. To run the frontend with hot-reloading on `http://localhost:3000/`, run:

```bash
npm run dev
```


:::caution
If you have opened this page previously, please remove all local store data for this page from your web browser, and hard-reload the page. 

For example in Chrome, go to Inspect → Application → Local Storage → http://localhost:3000/ → Clear All, and then reload.
:::
 

### Mainnet deployment

**Prior to starting the mainnet deployment process, ensure you have your identities and wallets set up for controlling the canisters correctly. This guide assumes that this work has been done in advance.**

- #### Step 1: Create the canisters:

```bash
dfx canister --network ic create "encrypted_notes_${BUILD_ENV}"
dfx canister --network ic create www
```


**`encrypted_notes_rust` will only work if you have the Rust toolchain installed.**

- #### Step 2: Build the canisters:

```bash
dfx build "encrypted_notes_${BUILD_ENV}" --network ic
dfx build www --network ic
```

- #### Step 3: Deploy to mainnet:


**In the commands below, --mode could also be reinstall to reset the stable memory.**

```bash
dfx canister --network ic install "encrypted_notes_${BUILD_ENV}" --mode=upgrade
dfx canister --network ic install www --mode=upgrade
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices, see also the [disclaimer](#disclaimer-please-read-carefully) above.  

For example, the following aspects are particularly relevant for this app: 
* [Make sure any action that only a specific user should be able to do requires authentication](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#make-sure-any-action-that-only-a-specific-user-should-be-able-to-do-requires-authentication), since a user should only be able to manage their own notes.
* [Protect key material against XSS using Web Crypto API](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#crypto-protect-key-material-against-xss-using-web-crypto-api), since this app stores private keys in the browser. 
* [Use secure cryptographic schemes](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#use-secure-cryptographic-schemes), since notes are being encrypted.

## User interaction with "Encrypted Notes" dapp

### Scenario I: Basic single-device usage

<p align="center">
  <img src="https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/pictures/single_user.png?raw=true" width="80%" height="80%"/>
</p>
<p align = "center">
Fig. 2. Basic single-device scenario for a user.
</p>

- #### Step 1: Open the main page of the `Encrypted Notes` dapp. You will see a _login_ button.

   1. If deployed locally, visit the following link: http://localhost:8000?canisterId=rkp4c-7iaaa-aaaaa-aaaca-cai
   2. If deployed to the mainnet IC, visit the corresponding canister URL.

   At this moment, only one _deviceAlias_ variable is stored in the _Local Storage_ (see Fig. 2(a)).

   **Note:** see [Troubleshooting](#troubleshooting) in case of problems.

- #### Step 2: Click the "Login" button. You will be redirected to the _Internet Identity_ canister (see Fig. 2(b)).

   1. If you already have an `anchor`, you may continue with it. Click "Authenticate", then verify your identity and finally click "Proceed", see Fig. 2(c).
   2. If you do not have an anchor yet, you should [create one](https://internetcomputer.org/how-it-works/web-authentication-identity/). Once an `anchor` is created, please follow 2.1.

- #### Step 3: Once logged in for the first time, your notes list should be empty.

At this moment, your _Local Storage_ should be populated with additional variables (see Fig. 2(d)): **ic-identity**, **ic-delegation**. 

These variables are used for storing/retrieving notes from the backend canister. 

In addition, another two variables are generated in the _IndexedDB_: **PrivateKey**, **PublicKey**. These two variables are used for encrypting/decrypting the shared secret key.

- #### Step 4: Create/edit/delete notes and observe changes in the resulting notes list (see Fig. 2(e)).

### Scenario II: the user is accessing notes from multiple devices

In this scenario, a user accesses the dapp using the same _Internet Identity_ anchor from multiple devices. From our dapp's perspective, each web browser instance can be viewed as a separate device.

<p align="center">
  <img src="https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/pictures/multiple_devices.png?raw=true" width="50%" height="50%"/>
</p>
<p align = "center">
Fig. 3. Scenario for a user with multiple registered devices.
</p>

- #### Step 1: Perform steps 1-3 of Scenario I on Device A.

- #### Step 2:. Perform steps 1-3 of Scenario I on Device B. 

One subtle difference that you might observe on Device B is that the message "Synchronizing..." (Fig. 3(a)) appears for a short time. As Device A was the first to log in, it was also the first one to generate a shared secret. Device B has to retrieve it. To do that, Device B first uploads its public key (pub B) to the backend canister. Device A retrieves pub B using periodic polling. Device A then re-encrypts the shared secret with pub B and uploads it to the backend. Afterward, Device B can retrieve the encrypted shared secret and decrypt it with its private key.

- #### Step 3: Observe that the list of notes is now empty for both devices.

- #### Step 4: Create a Note, e.g. "Note from Device A" on Device A, and observe it on Device B.

- #### Step 5: Analogously, create a different note, e.g. "Note from Device B" on Device B.

- #### Step 6: Confirm that the notes are synchronized between the two devices.

### Scenario III: device management

<p align="center">
  <img src="https://github.com/dfinity/examples/blob/master/motoko/encrypted-notes-dapp/pictures/registered_devices.png?raw=true" width="30%" height="30%"/>
</p>
<p align = "center">
Fig. 4. Scenario for a user adding/removing devices.
</p>

- #### Step 1: Login into the dapp with the same anchor on two or more devices.

- #### Step 2: On each device, navigate to "Devices" item in the menu.

- #### Step 3: Observe that the list of registered devices contains as many entries as the number of logged-in devices.

- #### Step 4: Assuming we are using Device A, click "remove" for some other device, say, Device B.

- #### Step 5: While still on Device A, observe that Device B is deleted from the list of devices.

 _Note_: a device cannot remove itself. That is why you do not see a "remove" button for your current device.

- #### Step 6: Switch to Device B and observe that it has been logged out.

- #### Step 7: Log in with Device B again and observe in "Device" tab both devices again.

## Unit testing

The unit tests are implemented in `src/encrypted_notes_motoko/test/test.mo` using the [Motoko Matchers](https://kritzcreek.github.io/motoko-matchers/) library. 

The easiest way to run all tests involves the following steps:

- #### Step 1: Follow the [above instructions](#option-1-docker-deployment) for Deployment via Docker with `BUILD_ENV=motoko`.

- #### Step 2:. Open a new console, type `docker ps`, and copy the _`<CONTAINER ID>`_ of the `encrypted_notes` image.

- #### Step 3: Run: `docker exec `_`<CONTAINER ID>`_` sh src/encrypted_notes_motoko/test/run_tests.sh`

- #### Step 4: Observer `All tests passed.` at the end of the output.


Alternatively, one can also run unit tests after a local deployment via:
```sh
src/encrypted_notes_motoko/test/run_tests.sh
```
However, this requires installing [`wasmtime`](https://wasmtime.dev/) and [`motoko-matchers`](https://github.com/kritzcreek/motoko-matchers):

```sh
git clone https://github.com/kritzcreek/motoko-matchers $(dfx cache show)/motoko-matchers
chmod +x src/encrypted_notes_motoko/test/run_tests.sh
src/encrypted_notes_motoko/test/run_tests.sh
```

Observer `All tests passed.` at the end of the output.


## Troubleshooting
### Building/deployment problems
Error `ERR_OSSL_EVP_UNSUPPORTED`.
Version 17+ of node.js introduces changes to the way Node handles OpenSSL.
This can cause conflicts with certain dependencies that require the old behavior.

Possible Remedies:
1. `export NODE_OPTIONS=--openssl-legacy-provider` (tested with node 17+)
2. Regress node version to 16.13.2 LTS (untested)

### Login problems
Some errors like `Could not initialize crypto service` might occur due to browser caching issues. Redeployment of the dapp can cause such problems. In this case clear the browser's `_Local Storage_` and `_IndexedDB_`.

### SSL certificate problems

Some browsers may block local resources based on invalid SSL certificates. If while testing a locally deployed version of the Encrypted Notes dapp you observe certificate issues in your browser's console, please change the browser settings to _ignore certificates for resources loaded from localhost_. For example, this can be done in Google Chrome via [chrome://flags/#allow-insecure-localhost](chrome://flags/#allow-insecure-localhost).

## dfx.json file structure
`dfx.json` is the configuration of the project when deploying to either the local replica or to the IC, it assists in the creation of the `.dfx` directory (which contains `canister_ids.json` — which merely maps canister by name to their id on both local replica and the IC). There are various configuration options here and this is not exhaustive. This will primarily discuss target types for canisters (which all exist under the `canisters` key).

```sh
{
    "canisters": {
        "encrypted_notes_motoko": {
            "main": "src/encrypted_notes_motoko/main.mo",
            "type": "motoko"
        },
        "encrypted_notes_rust": {
            "type": "custom",
            "build": "cargo build --target wasm32-unknown-unknown --package encrypted_notes_rust --release",
            "wasm": "target/wasm32-unknown-unknown/release/encrypted_notes_rust.wasm",
            "candid": "src/encrypted_notes_rust/src/encrypted_notes_rust.did"
        },
        "www": {
            "dependencies": ["encrypted_notes_motoko"],
            "frontend": {
                "entrypoint": "src/frontend/public/index.html"
            },
            "source": ["src/frontend/public/"],
            "type": "assets"
        },
        "internet_identity": {
            "candid": "internet_identity.did",
            "type": "custom",
            "wasm": "internet_identity.wasm"
        }
    },
    "networks": {
        "local": {
            "bind": "0.0.0.0:8000",
            "type": "ephemeral"
        }
    },
    "version": 1
}
```
**encrypted_notes_motoko**:
Motoko is the IC-specific language for building and deploying Canisters. Two keys are necessary:
`main`: The directory location of the entry point file of your canister.
`type`: needs to be "motoko", informing `dfx` of how to properly build the canister.

**encrypted_notes_rust**:
Rust natively supports WebAssembly — the binary format of the Internet Computer, and there is a crate ic_cdk that allows hooks into the IC. Unlike Motoko, `dfx` does not yet have a native Rust target that infers as much as Motoko canisters. So the keys that need to be provided are:
`type`: custom (letting `dfx` know that it's going to need to do some user-defined work)
`build`: whatever command is needed to turn your project into a Wasm binary. In this repo, it's:
```sh
cargo build --package encrypted_notes_rust --target wasm32-unknown-unknown --release
```
`wasm`: wherever the wasm binary ends up at the end of the "build" command.
`candid`: There is not yet Rust autogeneration for candid IDL built into `dfx`, so DFX needs to know where your Candid file for the canister built by "build" resides.
**www**:
frontend www canister (an "asset" canister) is the way we describe a set of files or a static website that we are deploying to the IC. Our project frontend is built in [Svelte](https://svelte.dev/). The keys we used are as follows:
`dependencies`: an array of whatever canisters are being used to serve your app, to ensure that `dfx` builds and deploys them before your app.
`frontend: { entrypoint: ""}`: This set of keys tells `dfx` to build it as a frontend canister, and entrypoint is wherever your app entrypoint winds up residing at the end of an npm build
`source`: where the rest of your app resides at the end of npm build
`type`: "assets" for an assets or static canister.  

**Binary targets**:
You can also just deploy arbitrary binary targets as long as they're wasm binaries. For that, use the keys:
`wasm`: a wasm file.
`candid`: a Candid file representing all interfaces in the wasm file.

:::info
If there is a mismatch between "wasm" and "candid" interface definitions, your canister will not deploy.
:::

---
&nbsp;

## Local memory model

:::info
This dapp uses the web browser's `_Local Storage_` and `_IndexedDB_` for storing the following data:

- Device name.
- User identity info.
- A private/public key pair.

:::

A symmetric key for encrypting/decrypting the notes is stored in RAM (this key is shared between multiple devices). For a better understanding of the mechanics of the dapp, please see the `_Local Storage_`/`_IndexedDB_` windows in your web browser. 

In Chrome, go to: _Developer Tools→Application→Local Storage_/_IndexedDB_.

---
&nbsp;


## Acknowledgments
We thank the author of [IC Notes](https://github.com/pattad/ic_notes) whose code was the starting point for the frontend component used in this project.

We thank the authors of [IC Vault](https://github.com/timohanke/icvault) whose code was the starting point for this project's backend, browser-based encryption, and device management.
