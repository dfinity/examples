import "../lib/init.ts";
import { get, writable } from "svelte/store";
import { AuthClient } from "@icp-sdk/auth/client";
import { DelegationIdentity } from "@icp-sdk/core/identity";
import { replace } from "svelte-spa-router";
import { createEncryptedMaps } from "../lib/encrypted_maps.js";
import { EncryptedMaps } from "@icp-sdk/vetkeys/encrypted_maps";

export type AuthState =
    | {
          state: "initializing-auth";
      }
    | {
          state: "anonymous";
          client: AuthClient;
      }
    | {
          state: "initialized";
          encryptedMaps: EncryptedMaps;
          client: AuthClient;
      }
    | {
          state: "error";
          error: string;
      };

export const auth = writable<AuthState>({
    state: "initializing-auth",
});

async function initAuth() {
    const isLocalEnv = window.location.hostname === "localhost" || window.location.hostname.endsWith(".localhost");
    const client = new AuthClient({
        identityProvider: isLocalEnv ? "http://id.ai.localhost:8000/#authorize" : undefined,
    });
    if (client.isAuthenticated()) {
        void authenticate(client);
    } else {
        auth.update(() => ({
            state: "anonymous",
            client,
        }));
    }
}

void initAuth();

export function login() {
    const currentAuth = get(auth);

    if (currentAuth.state === "anonymous") {
        void (async () => {
            try {
                await currentAuth.client.signIn({
                    maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
                });
                void authenticate(currentAuth.client);
            } catch (error: unknown) {
                console.error("Login failed:", error);
            }
        })();
    }
}

export async function logout() {
    const currentAuth = get(auth);

    if (currentAuth.state === "initialized") {
        await currentAuth.client.signOut();
        auth.update(() => ({
            state: "anonymous",
            client: currentAuth.client,
        }));
        void replace("/");
    }
}

export async function authenticate(client: AuthClient) {
    void handleSessionTimeout(client);

    try {
        const encryptedMaps = await createEncryptedMaps({
            identity: await client.getIdentity(),
        });

        auth.update(() => ({
            state: "initialized",
            encryptedMaps,
            client,
        }));
    } catch (e) {
        auth.update(() => ({
            state: "error",
            error: (e as Error).message || "An error occurred",
        }));
    }
}

// set a timer when the II session will expire and log the user out
async function handleSessionTimeout(client: AuthClient) {
    try {
        const identity = await client.getIdentity();
        if (!(identity instanceof DelegationIdentity)) return;

        const chain = identity.getDelegation();
        // expiration is a BigInt of nanoseconds since epoch
        const expirationMs =
            Number(chain.delegations[0].delegation.expiration) / 1_000_000;

        setTimeout(() => {
            void logout();
        }, expirationMs - Date.now());
    } catch {
        console.error("Could not handle delegation expiry.");
    }
}
