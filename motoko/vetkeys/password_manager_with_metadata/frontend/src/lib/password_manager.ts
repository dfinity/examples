import { Actor, HttpAgent, type ActorSubclass } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import type { Principal } from "@icp-sdk/core/principal";
import { EncryptedMaps } from "@icp-sdk/vetkeys/encrypted_maps";
import {
    idlFactory,
    type _SERVICE,
} from "../bindings/declarations/backend.did";
import { createEncryptedMaps } from "./encrypted_maps";
import { passwordFromContent, type PasswordModel } from "../lib/password";
import { vaultFromContent, type VaultModel } from "../lib/vault";

const canisterEnv = safeGetCanisterEnv<{
    "PUBLIC_CANISTER_ID:backend": string;
}>();

export class PasswordManager {
    /// The actor class representing the full interface of the canister.
    private readonly canisterClient: ActorSubclass<_SERVICE>;
    // TODO: inaccessible API are get, instert and remove
    readonly encryptedMaps: EncryptedMaps;

    constructor(
        canisterClient: ActorSubclass<_SERVICE>,
        encryptedMaps: EncryptedMaps,
    ) {
        this.canisterClient = canisterClient;
        this.encryptedMaps = encryptedMaps;
    }

    async setPassword(
        owner: Principal,
        vault: string,
        passwordName: string,
        password: Uint8Array,
        tags: string[],
        url: string,
    ): Promise<{ Ok: null } | { Err: string }> {
        const encryptedPassword = await this.encryptedMaps.encryptFor(
            owner,
            new TextEncoder().encode(vault),
            new TextEncoder().encode(passwordName),
            password,
        );
        const maybeError =
            await this.canisterClient.insert_encrypted_value_with_metadata(
                owner,
                stringToBytebuf(vault),
                stringToBytebuf(passwordName),
                { inner: encryptedPassword },
                tags,
                url,
            );
        if ("Err" in maybeError) {
            return maybeError;
        } else {
            return { Ok: null };
        }
    }

    async getDecryptedVaults(owner: Principal): Promise<VaultModel[]> {
        const vaultsSharedWithMe =
            await this.encryptedMaps.getAccessibleSharedMapNames();
        const vaultsOwnedByMeResult =
            await this.encryptedMaps.getOwnedNonEmptyMapNames();

        const vaultIds = new Array<[Principal, Uint8Array]>();
        for (const vaultName of vaultsOwnedByMeResult) {
            vaultIds.push([owner, vaultName]);
        }
        for (const [otherOwner, vaultName] of vaultsSharedWithMe) {
            vaultIds.push([otherOwner, vaultName]);
        }

        const vaults = [];

        for (const [otherOwner, vaultName] of vaultIds) {
            const result =
                await this.canisterClient.get_encrypted_values_for_map_with_metadata(
                    otherOwner,
                    { inner: vaultName },
                );
            if ("Err" in result) {
                throw new Error(result.Err);
            }

            const passwords = new Array<[string, PasswordModel]>();
            const vaultNameString = new TextDecoder().decode(vaultName);
            for (const [
                passwordNameBytebuf,
                encryptedData,
                passwordMetadata,
            ] of result.Ok) {
                const passwordNameBytes = Uint8Array.from(
                    passwordNameBytebuf.inner,
                );
                const passwordNameString = new TextDecoder().decode(
                    passwordNameBytes,
                );
                const data = await this.encryptedMaps.decryptFor(
                    otherOwner,
                    vaultName,
                    passwordNameBytes,
                    Uint8Array.from(encryptedData.inner),
                );

                const passwordContent = new TextDecoder().decode(data);
                const password = passwordFromContent(
                    otherOwner,
                    vaultNameString,
                    passwordNameString,
                    passwordContent,
                    passwordMetadata,
                );
                passwords.push([passwordNameString, password]);
            }

            const usersResult = await this.encryptedMaps
                .getSharedUserAccessForMap(otherOwner, vaultName)
                .catch(() => []);

            vaults.push(
                vaultFromContent(
                    otherOwner,
                    vaultNameString,
                    passwords,
                    usersResult,
                ),
            );
        }

        return vaults;
    }

    async removePassword(
        owner: Principal,
        vault: string,
        passwordName: string,
    ): Promise<{ Ok: null } | { Err: string }> {
        const maybeError =
            await this.canisterClient.remove_encrypted_value_with_metadata(
                owner,
                stringToBytebuf(vault),
                stringToBytebuf(passwordName),
            );
        if ("Err" in maybeError) {
            return maybeError;
        } else {
            return { Ok: null };
        }
    }
}

export async function createPasswordManager(agentOptions?: {
    identity?: HttpAgent["config"]["identity"];
}): Promise<PasswordManager> {
    const canisterId =
        canisterEnv?.["PUBLIC_CANISTER_ID:backend"];
    if (!canisterId) {
        throw new Error(
            "Canister ID for backend is not set",
        );
    }

    const agent = await HttpAgent.create({
        ...agentOptions,
        host: window.location.origin,
        rootKey: canisterEnv?.IC_ROOT_KEY,
    });

    const encryptedMaps = createEncryptedMaps(agent);
    const canisterClient = Actor.createActor<_SERVICE>(idlFactory, {
        agent,
        canisterId,
    });

    return new PasswordManager(canisterClient, encryptedMaps);
}

function stringToBytebuf(str: string): { inner: Uint8Array } {
    return { inner: new TextEncoder().encode(str) };
}
