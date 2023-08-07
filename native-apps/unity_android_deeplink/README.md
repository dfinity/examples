# Internet Identity Integration
These samples show one way to integrate Internet Identity with some native apps, like Unity Android games.

## ii_integration_page
It's an example that integrates with Internet Identity, with the backend and frontend. It derives from the [Internet Identity integration sample](https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration) demo with some modifications.
Please refer to [README](./ii_integration_page/README.md) for details.

## android_integration
This is a Unity project with [ICP.NET](https://github.com/BoomDAO/ICP.NET) embedded, which is a C# agent that is able to communicate with the IC from C#. Please refer to [README](./android_integration/README.md) for details. 

## Workflow
Here is the basic workflow that how to integrate with Internet Identity from a Unity Android game. The basic idea is to open the Web Browser from the game, login in with II in the browser, and pass the DelegationIdentity back to the game via DeepLink.

The steps in detail are described below:

1. Set up an [Internet Identity integration website](#ii_integration_page) which supports logging in with II.
2. Run a Unity game on Android, which is built from [android_integration sample](#android_integration).
3. Launch the Web Browser from the game to open the website set up in #1, with the public key of `Ed25519Identity` as a parameter.
4. Login with your Internet Identity in the Web Browser.
5. Launch the application via DeepLink, and pass the `DelegationIdentity` back to the game.
6. Call the backend canister with the `DelegationIdentity` to greet.

Please refer to the [ii_integration_page](./ii_integration_page/README.md) and [android_integration](./android_integration/README.md) samples to set up the website and build the Unity Android game.
