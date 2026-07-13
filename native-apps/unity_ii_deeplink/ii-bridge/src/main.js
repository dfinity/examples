import { AuthClient } from "@icp-sdk/auth/client";
import {
  DelegationChain,
  DelegationIdentity,
  ECDSAKeyIdentity,
  Ed25519PublicKey,
} from "@icp-sdk/core/identity";
import { hexToBytes } from "@noble/hashes/utils";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

// The ic_env cookie is set by the asset canister on every HTML response.
// It encodes the replica root key and every PUBLIC_CANISTER_ID:* variable.
// In dev mode the Vite server sets the same cookie (see vite.config.js).
const canisterEnv = safeGetCanisterEnv();
const backendCanisterId = canisterEnv?.["PUBLIC_CANISTER_ID:backend"];

if (!backendCanisterId) {
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' first."
  );
}

const agentDefaults = {
  host: window.location.origin,
  rootKey: canisterEnv?.IC_ROOT_KEY,
};

// Parse the Unity app's Ed25519 session key from the URL query parameter.
// Present only when this page is opened by the Unity app via SignIn().
// Each Unity session generates a fresh key, so the delegation chain must be
// recreated on every visit even when an existing II session is available.
let appPublicKey;
const sessionKeyParam = new URLSearchParams(window.location.search).get(
  "sessionkey"
);
if (sessionKeyParam) {
  try {
    // Unity passes PublicKey.ToDerEncoding() — a DER-encoded Ed25519 key.
    // Ed25519PublicKey.from(str) expects raw 32 bytes, so decode the DER wrapper
    // explicitly.
    appPublicKey = Ed25519PublicKey.fromDer(hexToBytes(sessionKeyParam));
  } catch (e) {
    console.error("Invalid sessionkey param:", e);
  }
} else {
  document.getElementById("no-session-key")?.removeAttribute("hidden");
}

const SESSION_DURATION_MS = 15 * 60 * 1000; // 15 minutes; maximum allowed by II is 30 days

let actor = createActor(backendCanisterId, { agentOptions: agentDefaults });
let delegationChain;

const statusEl = document.getElementById("login-status");

function setStatus(text) {
  statusEl.textContent = text;
}

// ---- Event handlers — registered synchronously so they are always in place ----
// IMPORTANT: All onclick handlers must be registered before any await, otherwise
// a top-level await suspends the module and buttons fall through to the default
// <form> submit, which navigates the page and strips query params.

document.getElementById("login").onclick = async (e) => {
  e.preventDefault();
  setStatus("Signing in…");

  try {
    // ---- Double-delegation bridge for native apps ----
    //
    // A direct II delegation cannot be forwarded safely to a native app via a
    // URL callback because:
    //   1. The II delegation is tied to the browser origin, not to any app key.
    //   2. Passing any long-lived credential through a URL exposes it to other
    //      apps registered for the same scheme (open redirect risk).
    //
    // Solution: generate a fresh ECDSA middle key in the browser (private key
    // never leaves the browser). Let II delegate to that key. Then create a
    // second, short-lived delegation from the middle key to the app's Ed25519
    // key. Only this second chain — which is useless without the original II
    // delegation — leaves the browser via the deep link.
    const middleKeyIdentity = await ECDSAKeyIdentity.generate();

    // AuthClient defaults to https://id.ai/authorize, which works both on
    // mainnet and locally (icp-cli >= 0.2.4 trusts mainnet II signatures).
    const authClient = new AuthClient({ identity: middleKeyIdentity });
    const middleIdentity = await authClient.signIn();

    // Swap the actor to use the authenticated identity so the Greet button
    // calls the backend as the logged-in principal.
    actor = createActor(backendCanisterId, {
      agentOptions: { ...agentDefaults, identity: middleIdentity },
    });
    setStatus("Signed in as: " + middleIdentity.getPrincipal().toText());
    document.getElementById("login").style.display = "none";

    // Chain a second delegation from the middle key to the app's Ed25519 key.
    // Only possible when this page was opened by the Unity app with ?sessionkey=.
    if (appPublicKey != null && middleIdentity instanceof DelegationIdentity) {
      delegationChain = await DelegationChain.create(
        middleKeyIdentity,
        appPublicKey,
        new Date(Date.now() + SESSION_DURATION_MS),
        { previous: middleIdentity.getDelegation() }
      );
    }
  } catch (e) {
    setStatus("Sign-in failed: " + e.message);
    console.error(e);
  }
};

document.getElementById("open").onclick = (e) => {
  e.preventDefault();

  if (!delegationChain) {
    document.getElementById("greeting").innerText = appPublicKey
      ? "Sign in with Internet Identity first, then tap this button."
      : "No session key found. Open this page from the Unity app — it passes a session key in the URL.";
    return;
  }

  // Pass the full delegation chain to the Unity app via the custom URL scheme.
  // The scheme is registered at build time by AndroidPostBuildProcessor.cs /
  // iOSPostBuildProcessor.cs.
  const url =
    "org.dfinity.unity-ii://authorize?delegation=" +
    encodeURIComponent(JSON.stringify(delegationChain.toJSON()));
  window.open(url, "_self");
};

document.getElementById("greet").onclick = async (e) => {
  e.preventDefault();

  const btn = e.currentTarget;
  btn.setAttribute("disabled", true);

  const greeting = await actor.greet();
  document.getElementById("greeting").innerText = greeting;

  btn.removeAttribute("disabled");
};

// ---- Async init — runs after handlers are registered ----
//
// Restores an existing II session from IndexedDB on every page load.
// This keeps the Greet button showing the authenticated principal even when
// the page was not opened via the Unity app (no sessionkey in URL).
//
// If a sessionkey IS present AND the session is still valid, pre-creates the
// delegation chain so "Return to App" works immediately
// without requiring a fresh sign-in.
//
// DelegationIdentity.sign() delegates to its inner ECDSA key, so the restored
// identity can act as the middle key in DelegationChain.create() — no need to
// store or restore the raw key separately.
(async () => {
  const existingClient = new AuthClient();
  if (!existingClient.isAuthenticated()) return;

  try {
    const existingIdentity = await existingClient.getIdentity();
    if (!(existingIdentity instanceof DelegationIdentity)) return;

    actor = createActor(backendCanisterId, {
      agentOptions: { ...agentDefaults, identity: existingIdentity },
    });
    setStatus("Signed in as: " + existingIdentity.getPrincipal().toText());
    document.getElementById("login").style.display = "none";

    if (appPublicKey != null) {
      try {
        delegationChain = await DelegationChain.create(
          existingIdentity,
          appPublicKey,
          new Date(Date.now() + SESSION_DURATION_MS),
          { previous: existingIdentity.getDelegation() }
        );
      } catch (e) {
        console.error("Failed to create delegation chain from existing session:", e);
      }
    }
  } catch (e) {
    console.error("Failed to restore existing session:", e);
  }
})();
