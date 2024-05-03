# Unity + Internet Identity universal links

## Overview
This sample shows one way to integrate Internet Identity with iOS apps. It contains two parts: a dapp with II integrated, and an Unity Project which interacts with this dapp.

[View this project on GitHub](https://github.com/dfinity/examples/tree/master/native-apps/unity_ii_universallink).

`ii_integration_dapp` is an example that integrates with Internet Identity, with the backend and frontend. It derives from the [Internet Identity integration sample](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications.
Please refer to [README](./ii_integration_dapp/README.md) for details.

`unity_project` is a Unity project with [ICP.NET](https://github.com/BoomDAO/ICP.NET) embedded, which is a C# agent that is able to communicate with the IC from C#. Please refer to [README](./unity_project/README.md) for details. 

## Workflow
Before continuing, please read through the [iOS Universal Links](https://developer.apple.com/ios/universal-links/) to understand how iOS Universal Links works.

Here is the basic workflow that how to integrate with Internet Identity from a Unity iOS game. The basic idea is to open the Web Browser from the game, login in with II in the browser, and pass the `DelegationChain` back to the game.

The steps in detail are described below:

1. Set up an [Internet Identity integration dapp](#ii_integration_dapp) which supports logging in with II, with an `apple-app-site-association` file associated.
   Please refer to [ii_integration_dapp](./ii_integration_dapp/README.md) to set up the dapp.

2. Run a Unity game on iOS, which is built from [ios_integration sample](#unity_project).
   Please refer to [unity_project](./unity_project/README.md) to build the Unity iOS game.

3. Launch the Web Browser from the game to open the dapp frontend deployed in #1, with the public key of `Ed25519Identity` as a parameter.

4. Login with your Internet Identity in the Web Browser.

5. Launch the application via App Links, and pass the `DelegationChain` back to the game as the URL parameter.

6. Composite the `DelegationIdentity` with `DelegationChain` and the `Ed25519Identity`.

7. Call the backend canister with the `DelegationIdentity` to greet.
