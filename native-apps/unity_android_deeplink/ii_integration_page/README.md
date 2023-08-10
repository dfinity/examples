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

If you never work on the IC and don't know how to deploy a dapp, please refer to the [Hello World sample](https://internetcomputer.org/docs/current/tutorials/deploy_sample_app) to learn the basic knowledge about the IC. Once you set up your IC developement environment locally, you can follow the below steps to deploy to the IC mainnet.

1. Enter the `ii_integration_page` directory from command line
2. Run `npm install` to install the npm packages
3. Run `dfx start --background`
4. Run `dfx deploy --network=ic --with-cycles=1000000000000`  
   Here we recommend to deploy the dapp to the IC mainnet as it's easier to access it from your Android devices. Or you can use the [deployed dapp](https://6x7nu-oaaaa-aaaan-qdaua-cai.icp0.io) instead.
