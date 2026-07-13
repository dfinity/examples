# Unity + Internet Identity

This example shows how a Unity mobile app can authenticate users through [Internet Identity](https://internetcomputer.org/internet-identity) and call an ICP canister using the resulting delegation identity.

The example consists of three parts:

- **Backend canister** (`backend/app.mo`) — a Motoko canister that returns a greeting including the caller's principal, proving the delegation was accepted.
- **II bridge canister** (`ii-bridge/`) — a small web page that runs in the mobile browser, handles the Internet Identity login, and forwards the resulting delegation to the Unity app via a deep link callback. **Each app must deploy and control its own instance** — using a shared or third-party deployment would mean trusting that party with your users' identity flow.
- **Unity project** (`unity_project/`) — a Unity app that opens the II bridge in a browser, receives the delegation via deep link, and calls the backend canister directly using [ICP.NET](https://github.com/edjCase/ICP.NET).

## How it works

Internet Identity's authorization protocol requires a signable key pair in the browser — a native app cannot participate in it directly. The II bridge implements a **double-delegation pattern** to bridge the gap:

1. The Unity app generates a fresh `Ed25519Identity` (session key) and opens the II bridge in the mobile browser, passing only its DER-encoded **public key** as `?sessionkey=<hex>`.
2. The II bridge generates a temporary `ECDSAKeyIdentity` (the *middle key*) whose **private key stays in the browser** and is non-extractable from WebCrypto.
3. Internet Identity delegates to the middle key (`II → middle key`) — this is the standard II authorization flow.
4. The II bridge uses the middle key to sign a second, short-lived delegation to the app's Ed25519 public key (`middle key → app key`).
5. The combined `DelegationChain` is URL-encoded and returned to the Unity app via the `org.dfinity.unity-ii://authorize#delegation=…` deep link. A URI fragment (`#`) is used so the delegation is not included in any HTTP request if the app is not installed and the OS falls back to opening the URL in a browser.
6. The Unity app constructs a `DelegationIdentity` from the chain and calls the backend canister directly via [ICP.NET](https://github.com/edjCase/ICP.NET), without going through the browser.

The backend receives the call with the user's II principal, verifying the full chain was valid.

**Why the second delegation?** After step 3, the first delegation (`II → middle key`) is only usable by whoever holds the middle key's private key — which is the browser, not the app. The second delegation converts this into a chain the app can use with its own Ed25519 private key. The app's private key never enters the browser; the middle key's private key never leaves it.

## Build and deploy

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/native-apps/unity_ii_deeplink
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`test.sh` verifies the backend canister responds correctly to an anonymous call.

Open the II bridge URL printed by `icp deploy` in a browser to exercise the delegation flow.

For hot-reload development of the II bridge:

```bash
npm run dev --prefix ii-bridge
```

## Testing locally with a mobile device

### What can be tested locally

| Step | Testable locally? | Notes |
|------|:-----------------:|-------|
| II login in mobile browser | ✅ Yes | Uses mainnet II at `https://id.ai/authorize` — icp-cli ≥ 0.2.4 trusts mainnet II signatures on the local replica |
| Deep link callback to Unity app | ✅ Yes | Custom URL scheme `org.dfinity.unity-ii://` requires no HTTPS |
| Backend canister call with delegation | ✅ Yes | Local replica accepts the mainnet-signed delegation chain |
| Android App Links (HTTPS variant) | ❌ No | Requires HTTPS with deployed `assetlinks.json`; Android verifies during install |
| iOS Universal Links (HTTPS variant) | ❌ No | Requires Apple developer account, deployed AASA file, and HTTPS |

### Setup for local testing

Two options are supported. See [TESTING.md](TESTING.md) for step-by-step instructions for both.

**Android emulator (recommended)**

Uses `adb reverse` to tunnel the local replica into the emulator — no WiFi or IP config needed:

```bash
icp network start -d
icp deploy
adb reverse tcp:8000 tcp:8000
```

(`adb` is in `$ANDROID_HOME/platform-tools`; add it to your PATH or use the full path.)

Set the Unity Inspector fields to:
- `Ii Bridge Url` → `http://ii-bridge.local.localhost:8000`
- `Greet Backend Canister` → output of `icp canister status backend -i`
- `Ic Gateway` → `http://localhost:8000`

**Physical device over WiFi**

Requires configuring the ICP gateway to accept requests from the device's LAN IP. The ICP HTTP gateway validates the `Host` header against an allowlist — a physical device connecting via `http://192.168.1.42:8000/` is rejected by default. Add a `networks` section to `icp.yaml` with your machine's LAN IP:

```yaml
networks:
  - name: local
    mode: managed
    gateway:
      bind: "0.0.0.0"
      port: 8000
      domains:
        - "192.168.1.42"   # replace with your LAN IP; do not commit
```

Then start and deploy normally:

```bash
icp network start -d
icp deploy
```

Set the Unity Inspector fields to:
- `Ii Bridge Url` → `http://192.168.1.42:8000/?canisterId=<ii-bridge-canister-id>`
- `Greet Backend Canister` → output of `icp canister status backend -i`
- `Ic Gateway` → `http://192.168.1.42:8000`

Find your LAN IP with `ipconfig getifaddr en0` (macOS), `hostname -I | awk '{print $1}'` (Linux), or `ipconfig` → IPv4 Address (Windows).

## Deploying to mainnet

```bash
icp deploy -e ic
```

After deployment, update the Unity Inspector fields:

```bash
icp canister status ii-bridge -i -e ic   # → Ii Bridge Url
icp canister status backend -i -e ic     # → Greet Backend Canister
```

Leave `icGateway` at the default `https://icp-api.io`.

## Upgrading to HTTPS deep links for production

The default example uses a **custom URL scheme** (`org.dfinity.unity-ii://`). Custom URL schemes work for local development but have a security limitation: any app installed on the device can register the same scheme and receive the delegation callback. The delegation chain is cryptographically bound to the app's Ed25519 private key so the data itself is useless to an interceptor, but the ICP security guidelines recommend eliminating the interception risk entirely for production apps.

The solution is to switch to **Android App Links** or **iOS Universal Links** — HTTPS-based mechanisms that cryptographically bind the callback URL to your specific app via a verification file hosted on your domain. These require mainnet deployment.

### Android App Links (HTTPS, mainnet only)

Android verifies the `assetlinks.json` file on your domain at install time and routes matching HTTPS URLs exclusively to your app.

**1. Deploy to mainnet and note the II bridge canister ID.**

**2. Update `AndroidPostBuildProcessor.cs`:**

Change the scheme constants and add `pathPrefix` and `autoVerify`:

```csharp
const string kAndroidScheme = "https";
const string kAndroidHost = "<your-canister-id>.icp0.io";
```

In `AppendAndroidIntentFilter`, add `autoVerify` to the intent filter and `pathPrefix` to the data node:

```csharp
var intentFilterNode = xmlDoc.CreateElement("intent-filter");
intentFilterNode.SetAttribute("autoVerify", kAndroidNamespaceURI, "true"); // add this

// ...existing action and category nodes...

var dataNode = xmlDoc.CreateElement("data");
dataNode.SetAttribute("scheme", kAndroidNamespaceURI, kAndroidScheme);
dataNode.SetAttribute("host", kAndroidNamespaceURI, kAndroidHost);
dataNode.SetAttribute("pathPrefix", kAndroidNamespaceURI, "/authorize"); // add this
```

**3. Create `ii-bridge/public/.well-known/assetlinks.json`:**

```json
[{
  "relation": ["delegate_permission/common.handle_all_urls"],
  "target": {
    "namespace": "android_app",
    "package_name": "<your.package.name>",
    "sha256_cert_fingerprints": ["<YOUR_SIGNING_CERT_SHA256_FINGERPRINT>"]
  }
}]
```

Get the fingerprint with: `keytool -list -v -keystore <your-keystore>.jks`

**4. Create `ii-bridge/public/.ic-assets.json5`** to allow the asset canister to serve the hidden directory:

```json5
[{"match": ".well-known/**", "allow_raw_access": true}]
```

**5. Update the callback URL in `ii-bridge/src/main.js`:**

```js
const url = "https://<your-canister-id>.icp0.io/authorize#delegation=" + ...
```

After redeployment, Android verifies `assetlinks.json` during app install. The OS routes `https://<your-canister-id>.icp0.io/authorize#delegation=…` exclusively to your app without showing a chooser dialog.

### iOS Universal Links (HTTPS, mainnet only)

iOS verifies the Apple App Site Association (AASA) file on your domain and registers the domain binding at app install time. Requires an Apple Developer account.

**1. Deploy to mainnet and note the II bridge canister ID.**

**2. Replace the body of `iOSPostBuildProcessor.cs`:**

Remove the `Info.plist` URL scheme patch and instead add an Associated Domains entitlement:

```csharp
#if UNITY_IOS
using System.IO;
using UnityEditor;
using UnityEditor.Callbacks;
using UnityEditor.iOS.Xcode;

public class iOSPostBuildProcessor
{
    const string kDomain = "<your-canister-id>.icp0.io";

    [PostProcessBuild]
    public static void OnPostprocessBuild(BuildTarget buildTarget, string path)
    {
        if (buildTarget != BuildTarget.iOS) return;
        AddAssociatedDomains(path);
    }

    static void AddAssociatedDomains(string buildPath)
    {
        // Write the entitlements file.
        const string kEntitlementsRelPath = "Unity-iPhone/Unity-iPhone.entitlements";
        var entitlementsFullPath = Path.Combine(buildPath, kEntitlementsRelPath);
        var entitlements = new PlistDocument();
        entitlements.root
            .CreateArray("com.apple.developer.associated-domains")
            .AddString("applinks:" + kDomain);
        entitlements.WriteToFile(entitlementsFullPath);

        // Register the entitlements file in the Xcode project.
        var pbxPath = PBXProject.GetPBXProjectPath(buildPath);
        var pbx = new PBXProject();
        pbx.ReadFromFile(pbxPath);
        var targetGuid = pbx.GetUnityMainTargetGuid();
        pbx.AddFileToBuild(targetGuid, pbx.AddFile(kEntitlementsRelPath, kEntitlementsRelPath));
        pbx.SetBuildProperty(targetGuid, "CODE_SIGN_ENTITLEMENTS", kEntitlementsRelPath);
        pbx.WriteToFile(pbxPath);
    }
}
#endif
```

**3. Create `ii-bridge/public/.well-known/apple-app-site-association`** (no `.json` extension):

```json
{
  "applinks": {
    "details": [{
      "appIDs": ["<TEAM_ID>.<BUNDLE_ID>"],
      "components": [{ "/": "/authorize*" }]
    }]
  }
}
```

Find your Team ID in the Apple Developer portal. Bundle ID is the one set in Unity Player Settings.

**4. Create `ii-bridge/public/.ic-assets.json5`** (same file as the Android step — works for both):

```json5
[{"match": ".well-known/**", "allow_raw_access": true}]
```

The asset canister must serve `apple-app-site-association` with `Content-Type: application/json`. The ICP asset canister infers this from the missing extension — verify with `curl -I https://<canister-id>.icp0.io/.well-known/apple-app-site-association`.

**5. Update the callback URL in `ii-bridge/src/main.js`:**

```js
const url = "https://<your-canister-id>.icp0.io/authorize#delegation=" + ...
```

After redeployment, iOS verifies the AASA file at app install time. The OS routes `https://<your-canister-id>.icp0.io/authorize#delegation=…` directly to your app — no other app can intercept it.

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, familiarize yourself with the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all best practices.

Key points for this delegation pattern:

- **Deploy your own II bridge.** The II bridge handles your users' II sessions and creates the delegation chain. Any party that controls the II bridge can issue delegations. Deploy and control your own instance — never point your app at a third-party bridge.
- The middle key's private key never leaves the browser — only the derived delegation chain is forwarded to the app.
- The second delegation has a configurable expiry (default: 15 minutes, set via `SESSION_DURATION_MS` in `ii-bridge/src/main.js`). The maximum allowed by II is 30 days. Use the shortest lifetime that is practical for your use case.
- The delegation chain is passed through a URL callback, which may appear in browser history and system logs.
- Custom URL schemes (`org.dfinity.unity-ii://`) can be intercepted by other apps on the same device. The delegation chain is cryptographically bound to your app's Ed25519 private key, so intercepting it without that key is useless — but scheme-based attacks remain a naming concern. For higher assurance in production, prefer Android App Links or iOS Universal Links.
