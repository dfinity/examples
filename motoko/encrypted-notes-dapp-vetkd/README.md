# Encrypted notes adapted for using vetKD

This is a copy of the encrypted-notes-dapp example, adapted to (1) use [the proposed vetKD feature](https://github.com/dfinity/interface-spec/pull/158) and (2) add sharing of notes between users.

In particular, instead of creating a principal-specific AES key and syncing it across devices (by means of device-specific RSA keys), the notes are encrypted with an AES key that is derived (directly in the browser) from a note-ID-specific vetKey obtained from the backend canister (in encrypted form, using an ephemeral transport key), which itself obtains it from the vetKD system API. This way, there is no need for any device management in the dapp, plus sharing of notes becomes possible.

The vetKey used to encrypt and decrypt a note is note-ID-specific (and not, for example, principal-specific) so as to enable the sharing of notes between users. The derived AES keys are stored as non-extractable CryptoKeys in an IndexedDB in the browser for efficiency so that they respective vetKey only has to be fetched from the server once. To improve the security even further, the vetKeys' derivation information could be adapted to include a (numeric) epoch that advances each time the list of users with which the note is shared is changed.

Currently, the only way to use this dapp is via manual local deployment (see below).

Please also see the [README of the original encrypted-notes-dapp](../encrypted-notes-dapp/README.md) for further details.

## Disclaimer

This example uses an [**insecure** implementation](../../rust/vetkd/src/system_api) of [the proposed vetKD system API](https://github.com/dfinity/interface-spec/pull/158) in a pre-compiled form via the [vetkd_system_api.wasm](./vetkd_system_api.wasm). **Do not use this in production or for sensitive data**! This example is solely provided **for demonstration purposes** to collect feedback on the mentioned vetKD system API.

## Manual local deployment
1. Choose which implementation to use by setting a respective environment variable. You can choose Motoko or Rust.
   
   For **Motoko** deployment use
   ```sh
   export BUILD_ENV=motoko
   ```
   For **Rust** deployment use
   ```sh
   export BUILD_ENV=rust
   ```
2. To generate `$BUILD_ENV`-specific files (i.e., Motoko or Rust) run:
   ```sh
   sh ./pre_deploy.sh
   ```
3. [Install DFX](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html). Please keep in mind the dfx cli currently only runs on Linux and macOS.
4. Install npm packages from the project root:
   ```sh
   npm install
   ```
   _Note_: see [Troubleshooting](#troubleshooting) in case of problems
5. In case DFX was already started before, run the following:
   ```sh
   dfx stop
   rm -rf .dfx
   ```
6. Run in a separate shell (it blocks the shell):
   ```sh
   dfx start --clean
   ```
   ⚠️ If you see an error `Failed to set socket of tcp builder to 0.0.0.0:8000`, make sure that the port `8000` is not occupied, e.g., by the previously run Docker command (you might want to stop the Docker deamon whatsoever for this step).
7. Install a local [Internet Identity (II)](https://wiki.internetcomputer.org/wiki/What_is_Internet_Identity) canister:
   _Note_: If you have multiple dfx identities set up, ensure you are using the identity you intend to use with the `--identity` flag.
   1. To install and deploy a canister run:
      ```sh
      dfx deploy internet_identity --argument '(null)'
      ```
   2. To print the Internet Identity URL, run:
      ```sh
      npm run print-dfx-ii
      ```
   3. Visit the URL from above and create at least one local internet identity.
7. Install the vetKD system API canister:
   1. Ensure the Canister SDK (dfx) uses the canister ID that is hard-coded in the backend canister Rust source code:
      ```sh
      dfx canister create vetkd_system_api --specified-id s55qq-oqaaa-aaaaa-aaakq-cai
      ```
   2. Install and deploy the canister:
      ```sh
      dfx deploy vetkd_system_api
      ```
8. Deploy the encrypted notes backend canister:
   ```sh
   dfx deploy "encrypted_notes_$BUILD_ENV"
   ```
   ⚠️ Before deploying the Rust canister, you should first run `rustup target add wasm32-unknown-unknown`.
9. Update the generated canister interface bindings: 
   ```sh
   dfx generate "encrypted_notes_$BUILD_ENV"
   ```
10. Deploy the frontend canister:
    ```sh
    dfx deploy www
    ```
    You can check its URL with `npm run print-dfx-www`.
11. Open the frontend:
    1. Start the local development server, which also supports hot-reloading:
       ```sh
       npm run dev
       ```
    2. Open the URL that is printed in the console output. Usually, this is [http://localhost:3000/](http://localhost:3000/).

       ⚠️ If you have opened this page previously, please remove all local store data for this page from your web browser, and hard-reload the page. For example in Chrome, go to Inspect → Application → Local Storage → `http://localhost:3000/` → Clear All, and then reload.

## Troubleshooting

If you run into issues, clearing all the application-specific IndexedDBs in the browser (which are used to store Internet Identity information and the derived non-extractable AES keys) might help fixing the issue.