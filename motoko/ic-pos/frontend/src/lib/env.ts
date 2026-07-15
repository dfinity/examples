import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

/**
 * Canister IDs and network configuration for the frontend.
 *
 * In production the asset canister injects the `PUBLIC_CANISTER_ID:*` values and
 * the replica root key into the `ic_env` cookie (configured via icp.yaml). In
 * dev the Vite server sets the same cookie (see vite.config.ts).
 */
type IcPosCanisterEnv = {
  readonly ["PUBLIC_CANISTER_ID:icpos"]: string;
  readonly ["PUBLIC_CANISTER_ID:icrc1_ledger"]: string;
  readonly ["PUBLIC_CANISTER_ID:icrc1_index"]: string;
  readonly ["PUBLIC_CANISTER_ID:internet_identity"]: string;
};

const env = safeGetCanisterEnv<IcPosCanisterEnv>();

function requireCanisterId(name: keyof IcPosCanisterEnv): string {
  const id = env?.[name];
  if (!id) {
    throw new Error(
      `Canister ID "${name}" not found. Deploy the canisters (./deploy.sh) and reload.`
    );
  }
  return id;
}

export const icposCanisterId = requireCanisterId("PUBLIC_CANISTER_ID:icpos");
export const icrc1LedgerCanisterId = requireCanisterId(
  "PUBLIC_CANISTER_ID:icrc1_ledger"
);
export const icrc1IndexCanisterId = requireCanisterId(
  "PUBLIC_CANISTER_ID:icrc1_index"
);
export const internetIdentityCanisterId = requireCanisterId(
  "PUBLIC_CANISTER_ID:internet_identity"
);

export const rootKey = env?.IC_ROOT_KEY;

export const host = window.location.origin;

export const isLocal = /localhost|127\.0\.0\.1/.test(window.location.hostname);

export const iiUrl = isLocal
  ? `http://${internetIdentityCanisterId}.localhost:8000`
  : "https://id.ai";
