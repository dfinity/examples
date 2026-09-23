import { get, writable } from "svelte/store";
import { type BackendActor, createActor } from "../lib/actor";
import { AuthClient } from "@icp-sdk/auth/client";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import type { Principal } from "@icp-sdk/core/principal";
import { CryptoService } from "../lib/crypto";
import { showError } from "./notifications";
import { push } from "svelte-spa-router";

const rootKey = safeGetCanisterEnv()?.IC_ROOT_KEY;

export type AuthState =
  | { state: "initializing-auth" }
  | { state: "anonymous"; actor: BackendActor; client: AuthClient }
  | { state: "initializing-crypto"; actor: BackendActor; client: AuthClient; principal: Principal }
  | { state: "synchronizing"; actor: BackendActor; client: AuthClient; principal: Principal }
  | {
      state: "initialized";
      actor: BackendActor;
      client: AuthClient;
      principal: Principal;
      crypto: CryptoService;
    }
  | { state: "error"; error: string };

export const auth = writable<AuthState>({ state: "initializing-auth" });

async function initAuth() {
  const isLocal =
    window.location.hostname === "localhost" ||
    window.location.hostname.endsWith(".localhost");
  // The client mints its delegations by calling the II canister, so its agent
  // needs the same host and root key as the backend actor.
  const client = new AuthClient({
    identityProvider: {
      authorizeUrl: isLocal
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
    authenticate(client);
  } else {
    const actor = await createActor();
    auth.update(() => ({
      state: "anonymous",
      actor,
      client,
    }));
  }
}

initAuth();

export async function login() {
  const currentAuth = get(auth);

  if (currentAuth.state === "anonymous") {
    await currentAuth.client.signIn();
    authenticate(currentAuth.client);
  }
}

export async function logout() {
  const currentAuth = get(auth);

  if (currentAuth.state === "initialized") {
    // Switch first so the session subscription does not sign out twice.
    auth.set({ state: "initializing-auth" });
    await currentAuth.client.signOut();
    const actor = await createActor();
    auth.update(() => ({
      state: "anonymous",
      actor,
      client: currentAuth.client,
    }));
    push("/");
  }
}

export async function authenticate(client: AuthClient) {
  try {
    const identity = await client.getIdentity();
    const principal = identity.getPrincipal();
    const actor = await createActor({ identity });

    auth.update(() => ({
      state: "initializing-crypto",
      actor,
      client,
      principal,
    }));

    const cryptoService = new CryptoService(actor);

    auth.update(() => ({
      state: "initialized",
      actor,
      client,
      principal,
      crypto: cryptoService,
    }));
  } catch (e: any) {
    auth.update(() => ({
      state: "error",
      error: e.message || "An error occurred",
    }));
  }
}
