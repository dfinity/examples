# Internet Identity integration sample

This example derives from the [internet_identity_integration](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications, please read its [README](https://github.com/dfinity/examples/blob/master/motoko/internet_identity_integration/README.md) before continue.

## Overview

This example shows a use case to support login with the two delegations on the `DelegationChain`.

As we described in [Internet Identity Integration](../README.md#workflow), users can log in with II from the game. Usually what they do is

1. Generate the `Ed25519KeyIdentity` supported by [ICP.NET](https://github.com/BoomDAO/ICP.NET) in the Unity game.
2. For security purposes, only pass the public key of the `Ed25519KeyIdentity` to the Web browser for login. And only the public key is necessary when creating a `DelegationChain`.
3. In [index.js](./src/greet_frontend/src/index.js), we describe how to 
    - log in with Internet Identity with the frontend generated session key
    - retrieve the public key of the `Ed25519Identity` from the URL parameter and create another delegation with it. 

With this, users don't need to pass the private key around, also they don't need to store the private key outside of the game as they can regenerate the key pairs for every session.

## Deployment

If you never work on the IC and don't know how to deploy a dapp, please refer to the [Hello World sample](https://internetcomputer.org/docs/current/developer-docs/getting-started/hello-world) to learn the basic knowledge about the IC. 

And this example is configured to launch the Unity Android/iOS app by opening the `internetidentity://authorize` URL, you can change the URL scheme as you want in [index.js](./src/greet_frontend/src/index.js).

Once you set up the IC development environment locally and update the example as you want, you can follow the below steps to deploy to the IC mainnet.

1. Enter the `ii_integration_dapp` directory from the command line
2. Run `npm install` to install the npm packages
3. Run `dfx start --background`
4. Run `dfx deploy --network=ic --with-cycles=1000000000000`  
   Here we recommend deploying the dapp to the IC mainnet as it's easier to access it from your mobile devices. Or you can use the [deployed dapp](https://qsgof-4qaaa-aaaan-qekqq-cai.icp0.io) instead.
