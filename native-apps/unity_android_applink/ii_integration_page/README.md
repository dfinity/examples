# Internet Identity integration sample

This example derives from the [internet_identity_integration](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications, please read its [README](https://github.com/dfinity/examples/blob/master/motoko/internet_identity_integration/README.md) before continue.

## Overview

This example shows a special case to support login with the `IncompleteEd25519KeyIdentity` which only contains the public key. The reason why we support this is all for security. 

As we described in [Internet Identity Integration](../README.md#workflow), users can log in with II from the game. Usually what they do is

1. Generate the `Ed25519KeyIdentity` supported by [ICP.NET](https://github.com/BoomDAO/ICP.NET) in the Unity game.
2. For security purposes, only pass the public key of the `Ed25519KeyIdentity` to the Web browser for login, this is where `IncompleteEd25519KeyIdentity` can be used for.
3. In [index.js](./src/greet_frontend/src/index.js), we describe how to retrieve the public key of the `Ed25519Identity` from the URL parameter, use it to instantiate an `IncompleteEd25519KeyIdentity`, and log in with Internet Identity. 

With this, users don't need to pass the private key around, also they don't need to store the private key outside of the game as they can regenerate the key pairs for every session.

## Deployment

If you never work on the IC and don't know how to deploy a dapp, please refer to the [Hello World sample](https://internetcomputer.org/docs/current/tutorials/deploy_sample_app) to learn the basic knowledge about the IC. 

### Set up the `assetlinks.json` file

In order to support Android App Links, the website needs to serve an `assetlinks.json` file under the `.well-known` directory. The `assetlinks.json` file contains the package name, sha256 certificate fingerprints etc., which will be used by the Android installer to verify if the apps should be launched to handle the URLs defined in the Android manifest.xml. This is an example of [assetlinks.json](./src/greet_frontend/assets/.well-known/assetlinks.json) file in this sample.

For how to generate an assetlinks.json file, please refer to the [Android App Links](https://developer.android.com/studio/write/app-link-indexing#associatesite) document.

One thing to remind is `.well-known` folder will be ingored while deploying to the IC, please add a `.ic-assets.json` file insider a directory listed in `sources` in the `dfx.json` file, with the below content. 

```
[
    {
        "match": ".well-known",
        "ignore": false
    }
]

```

Here is an example of [.ic-assets.json](./src/greet_frontend/assets/.ic-assets.json) in this sample.

### Deploy the Dapp

Once you set up the IC developement environment locally and update the example as you want, you can follow the below steps to deploy to the IC mainnet.

1. Enter the `ii_integration_page` directory from command line
2. Run `npm install` to install the npm packages
3. Run `dfx start --background`
4. Run `dfx deploy --network=ic --with-cycles=1000000000000`  
   Here we recommend to deploy the dapp to the IC mainnet as it's easier to access it from your Android devices. Or you can use the [deployed dapp](https://6x7nu-oaaaa-aaaan-qdaua-cai.icp0.io) instead.
