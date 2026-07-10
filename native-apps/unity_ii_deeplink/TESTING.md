# Testing Guide

## Level 1 — Backend only (no device, ~2 min)

**Status: ✅ Verified (2026-07-10)**

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

**Expected output from `test.sh`:**
```
=== Test 1: greet returns a greeting for the anonymous principal ===
("Hello, 2vxsx-fae!")
PASS
```

---

## Level 2 — Browser delegation flow (no Unity build, ~5 min)

**Status: ✅ Verified (2026-07-10)**

Confirms the II login and double-delegation logic work. Does **not** test the Unity app integration.

```bash
icp network start -d
icp deploy
# Open the frontend URL printed by icp deploy in a browser, e.g.:
#   http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:8000
```

**Steps and expected results:**

1. **Login with Internet Identity** — mainnet II popup opens (`https://id.ai`). After login the page shows `Logged in as: <your-principal>`.

2. **Greet** — calls the backend with the authenticated identity. Shows `Hello, <your-principal>!` (not `2vxsx-fae`). This proves the delegation chain was accepted by the local replica.

3. **Launch Application via Deep Link** — shows `No delegation chain. Open this page from the Unity app to use this button.`

   **This is expected.** The delegation chain is only created when there is a `?sessionkey=<hex>` query parameter in the URL — which the Unity app provides when it opens the browser. When you open the page directly in a browser there is no session key, so there is no chain to forward. The button is only functional in the Unity → browser → Unity flow (Level 3).

---

## Level 3 — Full device test (physical Android/iOS, ~30 min)

**Status: 🔲 Not yet verified**

Tests the complete Unity app → browser → deep link → canister call flow.

### Prerequisites

- Physical Android or iOS device on the same WiFi as your Mac
- Unity installed (tested with Unity 2022 LTS or later)
- The ICP.NET DLLs are already bundled in `unity_project/Assets/ICP.NET/`

### Steps

**1. Start the gateway on all network interfaces**

```bash
ICP_NETWORK_BIND_ADDRESS=0.0.0.0:8000 icp network start -d
icp deploy
```

Note your machine's LAN IP:
```bash
ipconfig getifaddr en0   # or en1 for WiFi
```

Get the canister IDs:
```bash
icp canister id frontend
icp canister id backend
```

**2. Configure Unity Inspector**

Open the Unity project (`unity_project/`) and select the `TestICPAgent` GameObject. In the Inspector set:

| Field | Value |
|-------|-------|
| `greetFrontend` | `http://<LAN-IP>:8000/?canisterId=<frontend-canister-id>` |
| `greetBackendCanister` | `<backend-canister-id>` |
| `icGateway` | `http://<LAN-IP>:8000` |

**3. Build and install to device**

Build and Run from Unity (File → Build Settings → select your platform → Build & Run).

The post-build processors register the `internetidentity://` custom URL scheme automatically:
- **Android**: `AndroidPostBuildProcessor.cs` injects `<intent-filter>` into `AndroidManifest.xml`
- **iOS**: `iOSPostBuildProcessor.cs` adds `CFBundleURLTypes` to `Info.plist`

**4. Run the flow on the device**

1. Tap the button in the Unity app that opens the browser — navigates to `http://<LAN-IP>:8000/?canisterId=...&sessionkey=<ed25519-hex>` (the `sessionkey` param is auto-appended by the app)
2. Tap **Login with Internet Identity** — mainnet II opens; log in with your passkey
3. Page shows `Logged in as: <your-principal>` (confirms login)
4. Tap **Launch Application via Deep Link** — browser fires `internetidentity://authorize?delegation=…`
5. OS routes the deep link back to the Unity app
6. Unity app shows `Hello, <your-principal>!` in the UI

---

## Level 4 — Mainnet

**Status: 🔲 Not yet verified**

```bash
icp deploy -e ic
icp canister id frontend -e ic   # → set as greetFrontend in Unity Inspector
icp canister id backend -e ic    # → set as greetBackendCanister
# Leave icGateway as https://ic0.app
```

Follow Level 3 steps from "Configure Unity Inspector" onward.

---

## Known limitations

| Variant | Local? | Notes |
|---------|:------:|-------|
| Custom URL scheme (deeplink) | ✅ | No HTTPS required |
| Android App Links | ❌ | Requires deployed `assetlinks.json` over HTTPS; Android verifies on install |
| iOS Universal Links | ❌ | Requires Apple Developer account, HTTPS AASA file |
