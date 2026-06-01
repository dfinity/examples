import { HttpAgent } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor as createEncryptedNotesActor, type Backend } from "../declarations/encrypted_notes/encrypted_notes_rust.did";

export type BackendActor = Backend;

const canisterEnv = safeGetCanisterEnv<{
  "PUBLIC_CANISTER_ID:encrypted_notes": string;
}>();

export async function createActor(options?: { identity?: any }): Promise<BackendActor> {
  const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:encrypted_notes"];
  const agent = await HttpAgent.create({
    identity: options?.identity,
    host: window.location.origin,
    ...(canisterEnv?.IC_ROOT_KEY ? { rootKey: canisterEnv.IC_ROOT_KEY } : {}),
  });
  return createEncryptedNotesActor(canisterId, { agent });
}
