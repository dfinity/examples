import { HttpAgent } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor as createEncryptedNotesActor, type Backend } from "../bindings/backend";

export type BackendActor = Backend;

const canisterEnv = safeGetCanisterEnv<{
  "PUBLIC_CANISTER_ID:backend": string;
}>();

export async function createActor(options?: { identity?: any }): Promise<BackendActor> {
  const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:backend"];
  const agent = await HttpAgent.create({
    identity: options?.identity,
    host: window.location.origin,
    rootKey: canisterEnv?.IC_ROOT_KEY,
  });
  return createEncryptedNotesActor(canisterId, { agent });
}
