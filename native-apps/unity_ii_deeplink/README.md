# Unity + Internet Identity

This example shows how a Unity mobile app can authenticate users through [Internet Identity](https://internetcomputer.org/internet-identity) and call an ICP canister using the resulting delegation identity.

The dapp consists of three parts:
- **Backend canister** (`backend/app.mo`) — a Motoko canister that returns a greeting including the caller's principal, proving the delegation was accepted.
- **Frontend canister** (`frontend/`) — a web page that runs in the mobile browser, performs the II login, and bridges the delegation to the Unity app via a URL callback.
- **Unity project** (`unity_project/`) — a Unity app that opens the frontend in a browser, receives the delegation via deep link, and calls the backend canister directly using [ICP.NET](https://github.com/BoomDAO/ICP.NET).

## How it works

Internet Identity issues a delegation that is tied to the browser origin — it cannot safely be forwarded to a native app as-is. Instead the frontend implements a **double-delegation bridge**:

1. The Unity app generates a fresh `Ed25519Identity` (session key) and opens the frontend in the mobile browser, appending its DER-encoded public key as `?sessionkey=<hex>`.
2. The frontend generates a short-lived `ECDSAKeyIdentity` (the *middle key*, private key never leaves the browser) and logs in with Internet Identity, which creates delegation `II → middle key`.
3. The frontend chains a second delegation: `middle key → app Ed25519 key` (15-minute expiry).
4. The combined `DelegationChain` is URL-encoded and returned to the Unity app via the `internetidentity://authorize?delegation=…` deep-link callback.
5. The Unity app constructs a `DelegationIdentity` from the chain and calls `backend` directly via [ICP.NET](https://github.com/BoomDAO/ICP.NET), without going through the browser.

The backend receives the call with the user's II principal, verifying the full chain was valid.

## Build and deploy from the command line

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

Open the frontend URL printed by `icp deploy` in a mobile browser to exercise the full delegation flow.

For hot-reload frontend development:

```bash
npm run dev --prefix frontend
```

## Testing locally with a mobile device

### What can be tested locally

| Step | Testable locally? | Notes |
|------|:-----------------:|-------|
| II login in mobile browser | ✅ Yes | Uses mainnet II at `https://id.ai/authorize` — icp-cli ≥ 0.2.4 trusts mainnet II signatures on the local replica |
| Deep link callback to Unity app | ✅ Yes | Custom URL scheme `internetidentity://` requires no HTTPS |
| Backend canister call with delegation | ✅ Yes | Local replica accepts the mainnet-signed delegation chain |
| Android App Links (HTTPS variant) | ❌ No | Requires HTTPS with deployed `assetlinks.json`; Android verifies during install |
| iOS Universal Links (HTTPS variant) | ❌ No | Requires Apple developer account, deployed AASA file, and HTTPS |

### Setup for local device testing

1. Start the gateway bound to all interfaces:
   ```bash
   ICP_NETWORK_BIND_ADDRESS=0.0.0.0:8000 icp network start -d
   icp deploy
   ```
2. Find your machine's LAN IP (e.g. `192.168.1.42`).
3. In Unity, open the `TestICPAgent` component in the Inspector and set:
   - `greetFrontend` → `http://192.168.1.42:8000/?canisterId=<frontend-canister-id>`  
     (get the ID with `icp canister id frontend`)
   - `greetBackendCanister` → `<backend-canister-id>`  
     (get with `icp canister id backend`)
   - `icGateway` → `http://192.168.1.42:8000`
4. Build the Unity app to your device. Your phone and computer must be on the same WiFi network.

## Deploying to mainnet

```bash
icp deploy -e ic
```

After deployment, update the Unity Inspector fields with the mainnet canister IDs:

```bash
icp canister id frontend -e ic   # → greetFrontend (as https://<id>.icp0.io/)
icp canister id backend -e ic    # → greetBackendCanister
```

Leave `icGateway` at the default `https://ic0.app`.

## Platform variants

The default example uses a **custom URL scheme** (`internetidentity://authorize`). Two HTTPS-based variants are possible for stronger security guarantees — they require mainnet deployment.

### Android App Links (HTTPS, mainnet only)

Android App Links use HTTPS instead of a custom scheme, preventing other apps from intercepting the callback URL.

1. Deploy to mainnet and note the frontend canister ID.
2. Update `AndroidPostBuildProcessor.cs`:
   - Replace the scheme constants:
     ```csharp
     const string kURLScheme = "https";
     const string kURLHost = "<your-canister-id>.icp0.io";
     const string kURLPath = "/authorize";
     ```
   - Add `autoVerify` to the intent filter:
     ```csharp
     intentFilterNode.SetAttribute("autoVerify", kAndroidNamespaceURI, "true");
     ```
3. Create `frontend/public/.well-known/assetlinks.json`:
   ```json
   [{"relation":["delegate_permission/common.handle_all_urls"],
     "target":{"namespace":"android_app","package_name":"<your.package>",
               "sha256_cert_fingerprints":["<YOUR_SHA256_FINGERPRINT>"]}}]
   ```
4. Create `frontend/public/.ic-assets.json5` to expose the hidden directory:
   ```json5
   [{"match":".well-known/**","allow_raw_access":true}]
   ```
5. In `frontend/src/main.js`, change the callback URL:
   ```js
   const url = "https://<canister-id>.icp0.io/authorize?delegation=" + ...
   ```

### iOS Universal Links (HTTPS, mainnet only)

iOS Universal Links use HTTPS and the `com.apple.developer.associated-domains` entitlement. Requires an Apple Developer account.

1. Deploy to mainnet.
2. Replace `iOSPostBuildProcessor.cs` with the universallink variant:
   - Use `PBXProject` to add an `.entitlements` file instead of patching `Info.plist`.
   - Set `kAssociatedDomainsKey = "com.apple.developer.associated-domains"` and add `applinks:<your-canister-id>.icp0.io`.
3. Serve an Apple App Site Association (AASA) file from the frontend canister at `/.well-known/apple-app-site-association`.
4. In `frontend/src/main.js`, change the callback URL to the HTTPS canister URL.

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, familiarize yourself with the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all best practices.

Key points for this delegation pattern:

- The ECDSA middle key's private key never leaves the browser — only the derived delegation chain is forwarded.
- The second delegation has a 15-minute expiry. Adjust via `new Date(Date.now() + <ms>)` in `main.js` (maximum allowed by II is 30 days).
- The delegation chain is passed through a URL callback, which may appear in browser history and system logs. Use the shortest session lifetime that is practical for your use case.
- Custom URL schemes (`internetidentity://`) can be intercepted by other apps on the same device. For higher assurance in production, prefer Android App Links or iOS Universal Links.
