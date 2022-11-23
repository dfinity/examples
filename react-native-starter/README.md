# React Native greet_app

This is a simple React Native project that seeks to communicate with a backend canister (dapp) deployed on the Internet Computer.
For the sake of demonstration, we are using a simplified Greet dapp -- which you can go to git submodule folder or this link [here:](https://github.com/wackyleo459/greet_dapp.git/).
Please git clone, and follow instructions to deploy.

## Environment Setup
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

3. Copy and paste the declared files from the `declarations` folder <em>of your DFX project</em> to the declarations folder in the root of this project. (For demonstration purpose this step is already done.)

4. From the dfx project `.dfx/local/canister_ids.json`, copy your canister id.

5. Paste that id into this project's `declarations/greet_dapp/index.js` file
    ```js 
    export const canisterId = <copied canisterId>
    ```

## React Native CLI with Metro
- `cd ios`
- `bundle install` to install Bundler
- `bundle exec pod install` to install iOS dependencies OR `pod install`

To start Metro bundler, have your Xcode open
- `cd ..`    to go back to root directory 
- `npm run ios` to build and start simulator for ios device


