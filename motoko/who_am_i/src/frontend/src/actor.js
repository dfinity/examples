import { AuthClient } from "@icp-sdk/auth/client";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

const canisterEnv = safeGetCanisterEnv();

const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:backend"];

if (!canisterId) {
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' first."
  );
}

const agentOptions = {
  rootKey: canisterEnv?.IC_ROOT_KEY,
};

// Internet Identity is deployed on the local network (`ii: true` in icp.yaml).
// REPLICA_PORT is injected by vite.config.js during `vite dev` since
// window.location.port would be the dev server port, not the network port.
const isLocal =
  window.location.hostname === "localhost" ||
  window.location.hostname === "127.0.0.1" ||
  window.location.hostname.endsWith(".localhost");
const networkPort = process.env.REPLICA_PORT || window.location.port;

// The client mints its delegations by calling the II canister, so its agent
// needs the network's root key to verify the responses.
export const authClient = new AuthClient({
  identityProvider: {
    authorizeUrl: isLocal
      ? `http://id.ai.localhost:${networkPort}/authorize`
      : "https://id.ai/authorize",
    canisterId: "rdmx6-jaaaa-aaaaa-aaadq-cai",
  },
  agentOptions: { rootKey: canisterEnv?.IC_ROOT_KEY },
});

export function createBackendActor(identity) {
  return createActor(canisterId, {
    agentOptions: { ...agentOptions, identity },
  });
}
