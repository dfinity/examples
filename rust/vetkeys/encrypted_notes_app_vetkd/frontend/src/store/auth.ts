import { get, writable } from "svelte/store";
import { type BackendActor, createActor } from "../lib/actor";
import { AuthClient } from "@icp-sdk/auth/client";
import { CryptoService } from "../lib/crypto";
import { showError } from "./notifications";
import { navigateTo } from "svelte-router-spa";

export type AuthState =
  | { state: "initializing-auth" }
  | { state: "anonymous"; actor: BackendActor; client: AuthClient }
  | { state: "initializing-crypto"; actor: BackendActor; client: AuthClient }
  | { state: "synchronizing"; actor: BackendActor; client: AuthClient }
  | { state: "initialized"; actor: BackendActor; client: AuthClient; crypto: CryptoService }
  | { state: "error"; error: string };

export const auth = writable<AuthState>({ state: "initializing-auth" });

async function initAuth() {
  const isLocal =
    window.location.hostname === "localhost" ||
    window.location.hostname.endsWith(".localhost");
  const client = new AuthClient({
    identityProvider: isLocal ? "http://id.ai.localhost:8000/authorize" : "https://id.ai/authorize",
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
    await currentAuth.client.signOut();
    const actor = await createActor();
    auth.update(() => ({
      state: "anonymous",
      actor,
      client: currentAuth.client,
    }));
    navigateTo("/");
  }
}

export async function authenticate(client: AuthClient) {
  handleSessionTimeout();

  try {
    const actor = await createActor({ identity: await client.getIdentity() });

    auth.update(() => ({
      state: "initializing-crypto",
      actor,
      client,
    }));

    const cryptoService = new CryptoService(actor);

    auth.update(() => ({
      state: "initialized",
      actor,
      client,
      crypto: cryptoService,
    }));
  } catch (e: any) {
    auth.update(() => ({
      state: "error",
      error: e.message || "An error occurred",
    }));
  }
}

function handleSessionTimeout() {
  setTimeout(() => {
    try {
      const delegation = JSON.parse(window.localStorage.getItem("ic-delegation") ?? "null") as {
        delegations: Array<{ delegation: { expiration: string } }>;
      } | null;
      if (!delegation) return;

      const expirationTimeMs =
        Number.parseInt(delegation.delegations[0].delegation.expiration, 16) / 1000000;

      setTimeout(() => {
        logout();
      }, expirationTimeMs - Date.now());
    } catch {
      console.error("Could not handle delegation expiry.");
    }
  });
}
