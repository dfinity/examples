# Encrypted notes adapted for using vetKD

This is a copy of the encrypted-notes-dapp example, adapted to use [the proposed vetKD feature](https://github.com/dfinity/interface-spec/pull/158).

In particular, instead of creating a principal-specific AES key and syncing it across devices (by means of device-specific RSA keys), the notes are encrypted with an AES key that is derived (directly in the browser) from a principal-specific vetKey obtained from the backend canister (in encrypted form, using an ephemeral transport key), which itself obtains it from the vetKD system API. This way, there is no need for any device management in the dapp.

The difference between the original encrypted-notes-dapp and the this one here can be seen in https://github.com/dfinity/examples/pull/561.

Please also see the [README of the original encrypted-notes-dapp](../encrypted-notes-dapp/README.md) for further details, especially the *disclaimer*.

Currently, the only way to deploy this app is with the following manual instructions (i.e., there are currently no instructions for using Docker).

## Manual local deployment
1. For **Motoko** deployment set environmental variable:
   ```sh
   export BUILD_ENV=motoko
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
