import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

const canisterEnv = safeGetCanisterEnv();

const canisterId =
  canisterEnv?.["PUBLIC_CANISTER_ID:backend"] ??
  process.env.CANISTER_ID_BACKEND;

if (!canisterId) {
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' first."
  );
}

const agentOptions = {
  host: window.location.origin,
  rootKey: canisterEnv?.IC_ROOT_KEY,
};

const isLocal =
  window.location.hostname === "localhost" ||
  window.location.hostname === "127.0.0.1" ||
  window.location.hostname.endsWith(".localhost");
const II_CANISTER_ID = "uqzsh-gqaaa-aaaaq-qaada-cai";
export const identityProviderUrl = isLocal
  ? `http://${II_CANISTER_ID}.localhost:${window.location.port || "8000"}`
  : "https://id.ai";

export function createBackendActor(identity) {
  return createActor(canisterId, {
    agentOptions: { ...agentOptions, identity },
  });
}
