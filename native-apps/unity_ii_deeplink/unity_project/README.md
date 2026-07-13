# Unity Project

This Unity project is the native app component of the [Unity + Internet Identity](../README.md) example. It demonstrates how to authenticate with Internet Identity from a mobile app and call an ICP canister using the resulting identity.

See the [main README](../README.md) for the full architecture, deployment instructions, and testing guide.

## How it works

The app uses a **double-delegation pattern** to authenticate users. Since Internet Identity runs in a browser and requires a signable key pair there, the app cannot interact with it directly. Instead:

1. The app generates a fresh `Ed25519Identity` (session key) on startup.
2. Tapping **Sign In with Internet Identity** opens the II bridge canister in the mobile browser, passing the app's Ed25519 public key as `?sessionkey=<hex>`.
3. The user completes the II login in the browser.
4. The browser sends the delegation chain back to the app via the `org.dfinity.unity-ii://` deep link.
5. The app constructs a `DelegationIdentity` from the chain and calls the backend canister directly using [ICP.NET](https://github.com/BoomDAO/ICP.NET).

## Key files

### Scripts

- [**`DeepLinkPlugin.cs`**](./Assets/Scripts/DeepLinkPlugin.cs) — Opens the II bridge in the browser (`SignIn()`), handles the deep link callback (`OnDeepLinkActivated()`), and parses the delegation chain JSON into a `DelegationIdentity`.

- [**`TestICPAgent.cs`**](./Assets/Scripts/TestICPAgent.cs) — Manages the `Ed25519Identity` and `DelegationIdentity`. Calls the backend canister via `GreetingClient`. Controls button state: hides the sign-in button after a delegation is received; re-shows it if the session expires.

- [**`DelegationUtils.cs`**](./Assets/Scripts/DelegationUtils.cs) — Model classes (`DelegationChainModel`, `SignedDelegationModel`, `DelegationModel`) used to deserialize the delegation chain JSON returned by the II bridge. All classes carry `[Serializable]` for Unity's `JsonUtility`.

- [**`GreetingClient.cs`**](./Assets/Scripts/GreetingClient.cs) — Auto-generated Candid client for the backend canister. Can be regenerated from `backend/backend.did` using the `ClientGenerator` in [ICP.NET](https://github.com/BoomDAO/ICP.NET).

### Post-build processors

- [**`AndroidPostBuildProcessor.cs`**](./Assets/Editor/AndroidPostBuildProcessor.cs) — Injects the `org.dfinity.unity-ii://` custom URL scheme into `AndroidManifest.xml` at build time so Android routes the deep link back to this app.

- [**`iOSPostBuildProcessor.cs`**](./Assets/Editor/iOSPostBuildProcessor.cs) — Registers the same custom URL scheme in `Info.plist` for iOS builds.

## Inspector configuration

Select the `AgentAndPlugin` GameObject in the scene hierarchy and set these fields on the `TestICPAgent` component before building:

| Field | Local (emulator) | Mainnet |
|-------|-----------------|---------|
| `Ii Bridge Url` | `http://ii-bridge.local.localhost:8000` | `https://<ii-bridge-canister-id>.icp0.io` |
| `Greet Backend Canister` | output of `icp canister id backend` | output of `icp canister id backend -e ic` |
| `Ic Gateway` | `http://localhost:8000` | `https://icp-api.io` (default) |

For local testing, run `adb reverse tcp:8000 tcp:8000` after starting the ICP network so the emulator's `localhost` resolves to the host machine. See [TESTING.md](../TESTING.md) for full setup instructions.

## Building

1. Open this directory (`unity_project/`) in Unity 2022 LTS or later.
2. Configure the Inspector fields above.
3. Go to **File → Build Settings**, select **Android** or **iOS**.
4. Click **Build & Run** to build and deploy to a connected device or emulator.

The post-build processors run automatically and register the deep link scheme — no manual manifest editing required.

## ICP.NET

The pre-compiled ICP.NET DLLs are bundled in `Assets/ICP.NET/`. This example uses version 4.0.0 from the [BoomDAO fork](https://github.com/BoomDAO/ICP.NET).
