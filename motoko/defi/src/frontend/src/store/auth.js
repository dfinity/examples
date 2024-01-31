import { writable } from "svelte/store";
import { idlFactory } from "../../declarations/defi_dapp/defi_dapp.did.js";
import { Actor, HttpAgent } from "@dfinity/agent";

/**
 * Creates an actor for the Backend canister
 *
 * @param {{agentOptions: import("@dfinity/agent").HttpAgentOptions, actorOptions: import("@dfinity/agent").ActorConfig}} options
 * @returns {import("@dfinity/agent").ActorSubclass<import("../../../declarations/defi_dapp/defi_dapp.did")._SERVICE>}
 */
export function createActor(options) {
  const hostOptions = {
    host:
      process.env.DFX_NETWORK === "ic"
        ? `https://${process.env.DEFI_DAPP_CANISTER_ID}.ic0.app`
        : "http://localhost:8000",
  };
  if (!options) {
    options = {
      agentOptions: hostOptions,
    };
  } else if (!options.agentOptions) {
    options.agentOptions = hostOptions;
  } else {
    options.agentOptions.host = hostOptions.host;
  }

  const agent = new HttpAgent({ ...options.agentOptions });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK === "local") {
    console.log('fetchRootKey')
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId: process.env.DEFI_DAPP_CANISTER_ID,
    ...options?.actorOptions,
  });
}

export const auth = writable({
  loggedIn: false,
   principal: '',
   actor: createActor(),
});


export const DEX_CANISTER_ID = process.env.DEFI_DAPP_CANISTER_ID;
export const AKITA_CANISTER_ID = process.env.AKITADIP20_CANISTER_ID;
export const GOLDENDIP20_CANISTER_ID = process.env.GOLDENDIP20_CANISTER_ID;
export const LEDGER_CANISTER_ID = process.env.LEDGER_CANISTER_ID;
export const whitelist = [DEX_CANISTER_ID, AKITA_CANISTER_ID, GOLDENDIP20_CANISTER_ID, LEDGER_CANISTER_ID];

export const host = process.env.DFX_NETWORK === "ic"
? `https://ic0.app`
    : `http://localhost:8000`

export const plugWallet = writable({
  extensionInstalled: false,
  isConnected: false,
  whiteList: whitelist,
  host: host,
  principal: '',
  plugActor: null,
  plugAkitaActor: null,
  plugGoldenActor: null,
  plugLedgerActor: null
});

export const anonymous = writable({
  actor: createActor()
});

