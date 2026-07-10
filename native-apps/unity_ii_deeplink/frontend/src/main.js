import { AuthClient } from "@icp-sdk/auth/client";
import {
  DelegationChain,
  DelegationIdentity,
  ECDSAKeyIdentity,
  Ed25519PublicKey,
} from "@icp-sdk/core/identity";
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
// The app passes its DER-encoded Ed25519 public key as a hex string so the
// frontend can chain a second delegation to it.
let appPublicKey;
const sessionKeyParam = new URLSearchParams(window.location.search).get(
  "sessionkey"
);
if (sessionKeyParam) {
  appPublicKey = Ed25519PublicKey.from(sessionKeyParam);
}

// Anonymous actor used before the user logs in.
let actor = createActor(backendCanisterId, { agentOptions: agentDefaults });
let delegationChain;

document.getElementById("login").onclick = async (e) => {
  e.preventDefault();

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

  // Chain a second delegation from the middle key to the app's Ed25519 key.
  if (appPublicKey != null && middleIdentity instanceof DelegationIdentity) {
    delegationChain = await DelegationChain.create(
      middleKeyIdentity,
      appPublicKey,
      new Date(Date.now() + 15 * 60 * 1000), // 15-minute session
      { previous: middleIdentity.getDelegation() }
    );
  }
};

document.getElementById("open").onclick = async (e) => {
  e.preventDefault();

  if (!delegationChain) {
    console.error("No delegation chain — log in with Internet Identity first.");
    return;
  }

  // Pass the full delegation chain to the Unity app via the custom URL scheme.
  // The scheme is registered at build time by AndroidPostBuildProcessor.cs /
  // iOSPostBuildProcessor.cs.
  const url =
    "internetidentity://authorize?delegation=" +
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
