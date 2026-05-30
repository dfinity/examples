import { Actor, HttpAgent, type ActorSubclass } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { idlFactory, type _SERVICE } from "../declarations/encrypted_notes/backend.did";

export type BackendActor = ActorSubclass<_SERVICE>;

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
  return Actor.createActor(idlFactory, { agent, canisterId });
}
