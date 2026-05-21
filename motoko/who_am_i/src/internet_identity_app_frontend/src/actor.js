import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { HttpAgent } from "@icp-sdk/core/agent";
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
// Local network: localhost variants and GitHub Codespaces port-forwarded URLs.
// Both require fetching the root key since they don't use the mainnet trust anchor.
function isLocalNetwork() {
  const { hostname } = window.location;
  return (
    hostname === "localhost" ||
    hostname === "127.0.0.1" ||
    hostname.endsWith(".localhost") ||
    hostname.endsWith(".app.github.dev")
  );
}

const II_CANISTER_ID = "uqzsh-gqaaa-aaaaq-qaada-cai";
const networkPort = process.env.REPLICA_PORT || window.location.port;
export const identityProviderUrl = isLocalNetwork()
  ? `http://${II_CANISTER_ID}.localhost:${networkPort}`
  : "https://id.ai";

export async function createBackendActor(identity) {
  const agent = HttpAgent.createSync({ ...agentOptions, identity });
  // When the ic_env cookie is absent (frontend served from the ICP gateway
  // directly rather than via Vite), rootKey is undefined and the agent would
  // fall back to the mainnet trust anchor. Fetch the actual root key instead.
  if (!canisterEnv?.IC_ROOT_KEY && isLocalNetwork()) {
    await agent.fetchRootKey();
  }
  return createActor(canisterId, { agent });
}
