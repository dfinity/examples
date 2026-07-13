# Testing Guide

## Level 1 — Backend only (no device, ~2 min)

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

Confirms the II login and double-delegation logic work. Does **not** test the Unity app integration.

```bash
icp network start -d
icp deploy
# Open the II bridge URL printed by icp deploy in a browser, e.g.:
#   http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:8000
```

**Steps and expected results:**

1. **Sign in with Internet Identity** — mainnet II opens (`https://id.ai`). After login the page shows `Signed in as: <your-principal>`.

2. **Greet** — calls the backend with the authenticated identity. Shows `Hello, <your-principal>!` (not `2vxsx-fae`). This proves the delegation chain was accepted by the local replica.

3. **Return to App** — shows `No session key found — open this page from the Unity app to complete the sign-in flow.`

   **This is expected.** The delegation chain is only created when a `?sessionkey=<hex>` query parameter is present — which the Unity app appends when it opens the browser. Without it there is no chain to forward, so the button does nothing useful. It is only functional in the full Unity → browser → Unity flow (Level 3).

---

## Level 3 — Full flow on Android emulator (~30 min)

Tests the complete Unity app → browser → deep link → canister call flow using the Android emulator and `adb reverse` to tunnel the local replica into it.

### Prerequisites

- Android Studio with an AVD (API 28+, with Play Store for passkey support)
- Unity 2022 LTS or later
- `adb` in your PATH (installed with Android Studio's SDK platform tools)
- The ICP.NET DLLs are already bundled in `unity_project/Assets/Plugins/ICP.NET/`

> If `adb` is not in your PATH, add `$ANDROID_HOME/platform-tools` to it, or use the full path:
> - **macOS**: `~/Library/Android/sdk/platform-tools/adb`
> - **Linux**: `~/Android/Sdk/platform-tools/adb`
> - **Windows**: `%LOCALAPPDATA%\Android\Sdk\platform-tools\adb.exe`

### Emulator settings

Before starting the AVD, configure it for Unity and Chrome compatibility:

| AVD setting | Value | Why |
|-------------|-------|-----|
| Graphics acceleration | **Software** | Chrome GPU process crashes on Vulkan/hardware acceleration |
| Graphics API (Unity Player Settings) | **OpenGL ES 3.0** | Unity render thread hangs on Vulkan/llvmpipe |

In Unity: **Edit → Project Settings → Player → Android → Other Settings → Graphics APIs**: remove Vulkan, add OpenGL ES 3.0.

### Steps

**1. Start the local network and deploy**

```bash
icp network start -d
icp deploy
```

**2. Tunnel the local replica into the emulator**

The emulator's `localhost` is isolated from the host machine. `adb reverse` bridges the gap so Chrome inside the emulator reaches the ICP replica on the host:

```bash
adb reverse tcp:8000 tcp:8000
```

This makes `http://localhost:8000` inside the emulator resolve to the host machine's `localhost:8000`.

> **Why not use `http://10.0.2.2:8000`?** The ICP HTTP gateway validates the `Host` header against a domain allowlist. `10.0.2.2` (the emulator's default alias for the host) is not in that list, so all requests return 400 "unknown domain". With `adb reverse`, Chrome uses `localhost` which is always in the allowlist.

**3. Configure Unity Inspector**

Open the Unity project (`unity_project/`) and select the `AgentAndPlugin` GameObject in the scene hierarchy. In the Inspector set:

| Field | Value |
|-------|-------|
| `Ii Bridge Url` | `http://ii-bridge.local.localhost:8000` |
| `Greet Backend Canister` | output of `icp canister status backend -i` |
| `Ic Gateway` | `http://localhost:8000` |

**4. Build and run**

Build and Run from Unity (File → Build Settings → Android → Build & Run). The post-build processor registers the `org.dfinity.unity-ii://` custom URL scheme automatically via `AndroidPostBuildProcessor.cs`.

**5. Passkeys on the emulator**

The Android emulator has no passkey storage by default. To create passkeys during II login you need to set a screen lock PIN and add a Google account:

1. **Settings → Security → Screen Lock → PIN** — set any PIN.
2. **Settings → Accounts → Add account → Google** — sign in with a Google account.

Without a Google account the passkey prompt falls back to "Use another device" (Bluetooth CTAP2), which is unavailable on the emulator.

**6. Run the flow**

1. Launch the app in the emulator.
2. Tap **Sign In with Internet Identity** — Chrome opens the II bridge at `http://ii-bridge.local.localhost:8000?sessionkey=<ed25519-hex>`.
3. Tap **Sign in with Internet Identity** on the browser page — mainnet II opens at `https://id.ai`; log in with your passkey.
4. Page shows `Signed in as: <your-principal>` and the sign-in button hides.
5. Tap **Return to App** — Chrome fires `org.dfinity.unity-ii://authorize#delegation=…`.
6. Android routes the deep link back to the Unity app.
7. The **Greet** button becomes active — tap it to call the backend.
8. The UI shows `Hello, <your-principal>!` — the same principal as the browser. ✅

---

## Level 3 (alternative) — Full flow on physical device (~45 min)

Tests the same flow on a physical Android or iOS device over WiFi. This requires extra configuration because the ICP HTTP gateway validates incoming `Host` headers — a physical device on the same WiFi connects via the machine's LAN IP (e.g. `192.168.1.42`), which is not in the default allowlist.

### Prerequisites

- Physical Android or iOS device on the same WiFi as the development machine
- The machine's LAN IP address (see step 1)

### Steps

**1. Find your LAN IP**

| OS | Command |
|----|---------|
| macOS | `ipconfig getifaddr en0` (or `en1` for WiFi) |
| Linux | `hostname -I \| awk '{print $1}'` |
| Windows | `ipconfig` → look for IPv4 Address |

**2. Configure the ICP gateway to accept requests from the LAN IP**

The gateway must explicitly allowlist the LAN IP. Add a `networks` section to `icp.yaml`:

```yaml
# Add to icp.yaml (replace 192.168.1.42 with your actual LAN IP)
networks:
  - name: local
    mode: managed
    gateway:
      bind: "0.0.0.0"
      port: 8000
      domains:
        - "192.168.1.42"
```

> This needs to be updated whenever the LAN IP changes (DHCP reassignment, different network). Do not commit this change to version control.

**3. Start the network and deploy**

```bash
icp network start -d
icp deploy
```

**4. Configure Unity Inspector**

| Field | Value |
|-------|-------|
| `Ii Bridge Url` | `http://192.168.1.42:8000/?canisterId=<ii-bridge-canister-id>` |
| `Greet Backend Canister` | output of `icp canister status backend -i` |
| `Ic Gateway` | `http://192.168.1.42:8000` |

**5. Build and install**

Build and Run from Unity (File → Build Settings → select your platform → Build & Run).

**6. Run the flow**

Same as the emulator flow (steps 2–8 above).

---

## Level 4 — Mainnet

```bash
icp deploy -e ic
icp canister status ii-bridge -i -e ic  # → set as Ii Bridge Url
icp canister status backend -i -e ic    # → set as Greet Backend Canister
# Leave Ic Gateway as https://icp-api.io (already the default)
```

Follow Level 3 steps from "Configure Unity Inspector" onward.

---

## Known limitations and platform notes

### Deep link mechanism

This example uses a **custom URL scheme** (`org.dfinity.unity-ii://`) for both Android and iOS. This is the only mechanism implemented here:

| Variant | This example | Local testing | Notes |
|---------|:------------:|:-------------:|-------|
| Custom URL scheme | ✅ implemented | ✅ works | No HTTPS required; first-come-first-served on device |
| Android App Links (`https://`) | ❌ not implemented | ❌ requires HTTPS | Needs `assetlinks.json` on live HTTPS domain; Android verifies at install |
| iOS Universal Links (`https://`) | ❌ not implemented | ❌ requires HTTPS | Needs Apple Developer account and live HTTPS AASA file |

Supporting App Links or Universal Links would require changing the post-build processors to register `https://` intent filters / associated domains entitlements, and deploying verification files to a live HTTPS server. This is intentionally out of scope for a local development example.

### Renaming the scheme for production

`org.dfinity.unity-ii://` is a dfinity-specific example scheme. For a production app use your own reverse-domain scheme (`com.yourcompany.yourapp://`). Three files must stay in sync:

- `Assets/Editor/AndroidPostBuildProcessor.cs` — `kAndroidScheme`
- `Assets/Editor/iOSPostBuildProcessor.cs` — `kURLScheme`
- `ii-bridge/src/main.js` — the URL string in the Return to App button handler

> **Security note:** custom URL schemes are first-come-first-served — any installed app can claim the same scheme and receive the callback URL. The delegation chain in the URL is cryptographically bound to your app's Ed25519 private key, so intercepting it without that key is useless. For higher assurance in production, Android App Links or iOS Universal Links eliminate the scheme-hijacking risk entirely.

### iOS status

The `iOSPostBuildProcessor.cs` correctly registers the custom URL scheme in `Info.plist` and Unity's `Application.deepLinkActivated` fires for custom URL schemes on iOS. The flow is expected to work on iOS, but **has not been tested** (Level 3 was only verified on Android).
