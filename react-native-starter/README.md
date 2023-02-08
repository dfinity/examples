# React Native greet_app

This is a simple React Native IOS project that seeks to communicate with a backend canister (dapp) deployed on the Internet Computer.

## Deploy Canister
Please follow instructions to deploy the backend canister first.
For the sake of demonstration, we are using a simplified `Greet dapp` which you can deploy directly at the root of this project. You can always choose to deploy a different canister and use that instead.

- `dfx deploy` the greet_dapp canister or your own canister, which can live separately outside this project.
- Once your canister is deployed, type in the terminal `dfx generate` to generate canister type declarations.

## Environment Setup for React Native
Plese refer to this document for details:
https://reactnative.dev/docs/environment-setup

Ensure following are installed
- Node
- Watchman
- Ruby (2.7.5) install specific version with Ruby version manager 
- Xcode (latest version most compatible with your OS) 
- Xcode Command Line Tools

## First Steps
1. Install packages via `npm install`

2. `npm run postinstall` to patch packages

3. Copy and paste the declared files from the `declarations` folder <em>of your DFX project</em> to `/src/declarations/greet_dapp` folder. 

   If you've dfx generated the given template `greet_dapp` from this project, this step will already be done for you.

4. Inside `index.js` file of declarations, create an exported actor with following `agentOptions` settings. The `./greet_dapp/reference_index.js` has the same pattern as reference:
    ``` js
    export const greet_dapp = createActor(canisterId, {
      agentOptions: {
        fetchOptions: {
          reactNative: {
            __nativeResponseType: "base64",
          },
        },
        callOptions: {
          reactNative: {
            textStreaming: true,
          },
        },
        host: "http://localhost:4943",
      },
    });
    ```

5. From the dfx project `.dfx/local/canister_ids.json`, copy your canister id.

6. Paste that id into the .env file like so:
    ```js
    GREET_DAPP_CANISTER_ID=rkp4c-7iaaa-aaaaa-aaaca-cai
    ```
    or paste into this project's `src/declarations/greet_dapp/index.js` file
    ```js 
    export const canisterId = <copied canisterId>
    ```

## React Native CLI with Metro

7. Pod installation
- `cd ios`
- `bundle install` to install Bundler
- `bundle exec pod install` OR `pod install` to install iOS dependencies 

8. To start Metro bundler, have your Xcode open
- `npm run ios` to build and start simulator for ios device


## Troubleshoot

If you have trouble running the application, try resetting the metro cache
by running `npm run reset`.

## Note on compatibility

There are a few libraries needed to make RN work with agent-js, including `react-native-fetch-api` and `fast-text-encoding`. You can simply import them into your entry file `index.js`:
```js
import "react-native-polyfill-globals/auto";
import "react-native-fetch-api";
import "fast-text-encoding";
```

Also, please modify the backend canister actor with agentOptions of `fetchOptions` and `callOptions` as shown above in step 4.

# Started app

<img src="./src/assets/simulator_screenshot.png?raw=true" width="400">