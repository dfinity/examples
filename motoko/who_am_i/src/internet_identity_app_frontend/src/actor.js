import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/internet_identity_app_backend";

const canisterEnv = safeGetCanisterEnv();

const canisterId =
  canisterEnv?.["PUBLIC_CANISTER_ID:internet_identity_app_backend"] ??
  process.env.CANISTER_ID_INTERNET_IDENTITY_APP_BACKEND;

if (!canisterId) {
  throw new Error(
    "Canister ID for 'internet_identity_app_backend' not found. Run 'icp deploy' or 'dfx deploy' first."
  );
}

// In GitHub Codespaces the Vite dev server runs on port 5173, but the ICP
// gateway only accepts requests whose Host matches its own port (8000). Derive
// the port-8000 forwarded URL from the current hostname so API calls bypass
// the Vite proxy and reach the gateway directly.
function getNetworkHost() {
  const { hostname, origin } = window.location;
  if (hostname.endsWith(".app.github.dev")) {
    return "https://" + hostname.replace(/-\d+\.app\.github\.dev$/, "-8000.app.github.dev");
  }
  return origin;
}

const agentOptions = {
  host: getNetworkHost(),
  rootKey: canisterEnv?.IC_ROOT_KEY,
};

// Resolve Internet Identity provider URL.
// II is always deployed on the local network (not the dev server).
// REPLICA_PORT is injected by vite.config.js during `vite dev` since
// window.location.port would be the dev server port, not the network port.
const isLocal =
  window.location.hostname === "localhost" ||
  window.location.hostname === "127.0.0.1" ||
  window.location.hostname.endsWith(".localhost");
const II_CANISTER_ID = "uqzsh-gqaaa-aaaaq-qaada-cai";
const networkPort = process.env.REPLICA_PORT || window.location.port;
export const identityProviderUrl = isLocal
  ? `http://${II_CANISTER_ID}.localhost:${networkPort}`
  : "https://id.ai";

export function createBackendActor(identity) {
  return createActor(canisterId, {
    agentOptions: { ...agentOptions, identity },
  });
}
