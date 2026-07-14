import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

// The ic_env cookie is set by the asset canister on all HTML responses. It
// contains the replica root key and PUBLIC_* canister environment variables.
// In dev mode the Vite dev server sets the same cookie (see vite.config.js).
const canisterEnv = safeGetCanisterEnv();

const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:backend"];

if (!canisterId) {
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' first."
  );
}

export const backend = createActor(canisterId, {
  agentOptions: {
    host: window.location.origin,
    rootKey: canisterEnv?.IC_ROOT_KEY,
  },
});
