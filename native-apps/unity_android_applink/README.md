# Internet Identity Integration
This sample shows one way to integrate Internet Identity with some native apps, like Unity Android games.

## ii_integration_page
It's an example that integrates with Internet Identity, with the backend and frontend. It derives from the [Internet Identity integration sample](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications.
Please refer to [README](./ii_integration_page/README.md) for details.

## android_integration
This is a Unity project with [ICP.NET](https://github.com/BoomDAO/ICP.NET) embedded, which is a C# agent that is able to communicate with the IC from C#. Please refer to [README](./android_integration/README.md) for details. 

## Workflow
Before continue, please read through the [Android App Links](https://developer.android.com/studio/write/app-link-indexing) to understand how Android App Links works.

Here is the basic workflow that how to integrate with Internet Identity from a Unity Android game. The basic idea is to open the Web Browser from the game, login in with II in the browser, and pass the DelegationIdentity back to the game.

The steps in detail are described below:

1. Set up an [Internet Identity integration dapp](#ii_integration_page) which supports logging in with II, with an `assetlinks.json` file associated.
   Please refer to [ii_integration_page](./ii_integration_page/README.md) to set up the dapp.
2. Run a Unity game on Android, which is built from [android_integration sample](#android_integration).
   Please refer to [android_integration](./android_integration/README.md) to build the Unity Android game.
3. Launch the Web Browser from the game to open the dapp frontend deployed in #1, with the public key of `Ed25519Identity` as a parameter.
4. Login with your Internet Identity in the Web Browser.
5. Launch the application via App Links, and pass the `DelegationIdentity` back to the game as the URL parameters.
6. Call the backend canister with the `DelegationIdentity` to greet.
