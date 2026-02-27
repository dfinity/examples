import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

// The ic_env cookie is set by the asset canister (SDK ≥0.30.2) on all HTML
// responses. It contains the replica root key and any PUBLIC_* canister
// environment variables. In dev mode the vite dev server sets the same cookie
// via Set-Cookie header (see vite.config.js).
const canisterEnv = safeGetCanisterEnv();

// Resolve canister ID: cookie (icp-cli + dev server) → env var (dfx build-time)
const canisterId =
  canisterEnv?.["PUBLIC_CANISTER_ID:backend"] ??
  process.env.CANISTER_ID_BACKEND;

if (!canisterId) {
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' or 'dfx deploy' first."
  );
}

export const backend = createActor(canisterId, {
  agentOptions: {
    host: window.location.origin,
    rootKey: canisterEnv?.IC_ROOT_KEY,
  },
});
