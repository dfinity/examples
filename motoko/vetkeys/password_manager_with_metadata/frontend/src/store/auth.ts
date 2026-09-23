import { get, writable } from "svelte/store";
import { AuthClient } from "@icp-sdk/auth/client";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import type { Principal } from "@icp-sdk/core/principal";
import { replace } from "svelte-spa-router";
import {
    PasswordManager,
    createPasswordManager,
} from "../lib/password_manager.js";

const rootKey = safeGetCanisterEnv()?.IC_ROOT_KEY;

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
          passwordManager: PasswordManager;
          client: AuthClient;
          principal: Principal;
      }
    | {
          state: "error";
          error: string;
      };

export const auth = writable<AuthState>({
    state: "initializing-auth",
});

async function initAuth() {
    const isLocalEnv =
        window.location.hostname === "localhost" ||
        window.location.hostname.endsWith(".localhost");
    // The client mints its delegations by calling the II canister, so its agent
    // needs the same host and root key as the backend actor.
    const client = new AuthClient({
        identityProvider: {
            authorizeUrl: isLocalEnv
                ? "http://id.ai.localhost:8000/authorize"
                : "https://id.ai/authorize",
            canisterId: "rdmx6-jaaaa-aaaaa-aaadq-cai",
        },
        agentOptions: { host: window.location.origin, rootKey },
    });
    // Leave the signed-in views when the session ends, including from another tab.
    client.subscribe(() => {
        if (!client.isAuthenticated()) void logout();
    });
    if (client.isAuthenticated()) {
        await authenticate(client);
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
        // Switch first so the session subscription does not sign out twice.
        auth.update(() => ({
            state: "anonymous",
            client: currentAuth.client,
        }));
        await currentAuth.client.signOut();
        await replace("/");
    }
}

export async function authenticate(client: AuthClient) {
    try {
        const identity = await client.getIdentity();
        const passwordManager = await createPasswordManager({ identity });

        auth.update(() => ({
            state: "initialized",
            passwordManager,
            client,
            principal: identity.getPrincipal(),
        }));
    } catch (e) {
        auth.update(() => ({
            state: "error",
            error: (e as Error).message || "An error occurred",
        }));
    }
}
