import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

/**
 * Canister IDs and network configuration for the frontend.
 *
 * In production the asset canister injects the `PUBLIC_CANISTER_ID:*` values and
 * the replica root key into the `ic_env` cookie (configured via icp.yaml). In
 * dev the Vite server sets the same cookie (see vite.config.ts).
 */
type CanisterEnv = {
  readonly ["PUBLIC_CANISTER_ID:backend"]: string;
  readonly ["PUBLIC_CANISTER_ID:icrc1_ledger"]: string;
  readonly ["PUBLIC_CANISTER_ID:icrc1_index"]: string;
};

const env = safeGetCanisterEnv<CanisterEnv>();

function requireCanisterId(name: keyof CanisterEnv): string {
  const id = env?.[name];
  if (!id) {
    throw new Error(
      `Canister ID "${name}" not found. Deploy the canisters (./deploy.sh) and reload.`
    );
  }
  return id;
}

export const backendCanisterId = requireCanisterId("PUBLIC_CANISTER_ID:backend");
export const icrc1LedgerCanisterId = requireCanisterId(
  "PUBLIC_CANISTER_ID:icrc1_ledger"
);
export const icrc1IndexCanisterId = requireCanisterId(
  "PUBLIC_CANISTER_ID:icrc1_index"
);
export const rootKey = env?.IC_ROOT_KEY;

export const host = window.location.origin;

export const isLocal = /localhost|127\.0\.0\.1/.test(window.location.hostname);

// Internet Identity provider. Locally the network serves II at
// id.ai.localhost (icp.yaml `ii: true`); on mainnet it's id.ai. The
// `/authorize` path is required by @icp-sdk/auth.
export const iiUrl = isLocal
  ? "http://id.ai.localhost:8000/authorize"
  : "https://id.ai/authorize";
