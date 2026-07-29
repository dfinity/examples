import { HttpAgent } from "@icp-sdk/core/agent";
import type { Principal } from "@icp-sdk/core/principal";
import {
    DefaultEncryptedMapsClient,
    EncryptedMaps,
    IndexedDbDerivedKeyMaterialCache,
} from "@icp-sdk/vetkeys/encrypted_maps";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";

const canisterEnv = safeGetCanisterEnv<{
    "PUBLIC_CANISTER_ID:backend": string;
}>();

export function createEncryptedMaps(
    agent: HttpAgent,
    principal: Principal,
): EncryptedMaps {
    const canisterId =
        canisterEnv?.["PUBLIC_CANISTER_ID:backend"];
    if (!canisterId) {
        throw new Error(
            "Canister ID for backend is not set",
        );
    }

    // Since v0.5.0 EncryptedMaps caches derived key material in memory only by
    // default, so it is discarded on page reload. We opt back into persisting
    // it across reloads with IndexedDbDerivedKeyMaterialCache, namespacing the
    // store by the caller's principal so one identity's cached keys are never
    // served to another on the same origin. Remember to call clearCache() on
    // logout / identity change (see the auth store).
    const cache = new IndexedDbDerivedKeyMaterialCache(
        `vetkeys-${principal.toText()}`,
    );

    return new EncryptedMaps(
        new DefaultEncryptedMapsClient(agent, canisterId),
        { cache },
    );
}
