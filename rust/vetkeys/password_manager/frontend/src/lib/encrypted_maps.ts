import "./init.ts";
import { HttpAgent, type HttpAgentOptions } from "@icp-sdk/core/agent";
import {
    DefaultEncryptedMapsClient,
    EncryptedMaps,
} from "@icp-sdk/vetkeys/encrypted_maps";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

const canisterEnv = safeGetCanisterEnv<{
    "PUBLIC_CANISTER_ID:ic_vetkeys_encrypted_maps_canister": string;
}>();

export async function createEncryptedMaps(
    agentOptions?: HttpAgentOptions,
): Promise<EncryptedMaps> {
    const canisterId =
        canisterEnv?.["PUBLIC_CANISTER_ID:ic_vetkeys_encrypted_maps_canister"];
    if (!canisterId) {
        throw new Error(
            "Canister ID for ic_vetkeys_encrypted_maps_canister is not set",
        );
    }

    const agent = await HttpAgent.create({
        ...agentOptions,
        host: window.location.origin,
        rootKey: canisterEnv?.IC_ROOT_KEY,
    });

    return new EncryptedMaps(new DefaultEncryptedMapsClient(agent, canisterId));
}
