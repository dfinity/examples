import { HttpAgent } from "@icp-sdk/core/agent";
import {
    DefaultEncryptedMapsClient,
    EncryptedMaps,
} from "@icp-sdk/vetkeys/encrypted_maps";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

const canisterEnv = safeGetCanisterEnv<{
    "PUBLIC_CANISTER_ID:password_manager_with_metadata": string;
}>();

export function createEncryptedMaps(agent: HttpAgent): EncryptedMaps {
    const canisterId =
        canisterEnv?.["PUBLIC_CANISTER_ID:password_manager_with_metadata"];
    if (!canisterId) {
        throw new Error(
            "Canister ID for password_manager_with_metadata is not set",
        );
    }

    return new EncryptedMaps(new DefaultEncryptedMapsClient(agent, canisterId));
}
