# Internet Identity integration sample

This example derives from the [internet_identity_integration](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications, please read its [README](https://github.com/dfinity/examples/blob/master/motoko/internet_identity_integration/README.md) before continue.

## Overview

This example shows a special case to support login with the `ImcompleteEd25519KeyIdentity` which only contains the public key. The reason why we support this is all for security. 

As we described in [Internet Identity Integration](/examples/native_apps/unity_android_deeplink/README.md#workflow), users can log in with II from the game. Usually what they do is

1. Generate the `Ed25519KeyIdentity` supported by [ICP.NET](https://github.com/BoomDAO/ICP.NET) in the Unity game.
2. For security purposes, only the public key of the `Ed25519KeyIdentity` to the Web browser for login, this is where `ImcompleteEd25519KeyIdentity` can be used for.
3. In [index.js](./src/greet_frontend/src/index.js), we describe how to retrieve the public key of an Ed25519Identity from the URL parameter, use it to instantiate an `ImcompleteEd25519KeyIdentity`, and log in with Internet Identity. 

With this, users don't need to pass the private key around, also they don't need to store the private key outside of the game as they can regenerate the key pairs for every session.
