import { execSync } from "child_process";

import type {
  ActorConfig,
  ActorSubclass,
  Agent,
  HttpAgentOptions,
  Identity,
} from "@dfinity/agent";
import { IDL } from "@dfinity/candid";
import { Principal } from "@dfinity/principal";
import { Actor, HttpAgent } from "@dfinity/agent";
import fetch from "isomorphic-fetch";

import {
  _SERVICE as Swap,
  idlFactory as swapIdlFactory,
} from "../../src/declarations/swap/swap.did.js";
import {
  _SERVICE as Token,
  idlFactory as tokenIdlFactory,
} from "../../src/declarations/token_a/token_a.did.js";

export declare interface CreateActorOptions {
  /**
   * @see {@link Agent}
   */
  agent?: Agent;
  /**
   * @see {@link HttpAgentOptions}
   */
  agentOptions?: HttpAgentOptions;
  /**
   * @see {@link ActorConfig}
   */
  actorOptions?: ActorConfig;
}

export function createActor<T>(
  canisterId: string | Principal,
  idlFactory: IDL.InterfaceFactory,
  options: CreateActorOptions = {},
): ActorSubclass<T> {
  const agent = options.agent || new HttpAgent({ ...options.agentOptions });

  if (options.agent && options.agentOptions) {
    console.warn(
      "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent.",
    );
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options.actorOptions,
  });
}

// Ask dfx where the the replica is running. This is a total hack to work
// around `dfx start` launching on a random port each time.
const dfxPort = execSync("dfx info replica-port", { encoding: "utf-8" });

export function agent(identity?: Identity) {
  const a = new HttpAgent({
    identity,
    host: `http://127.0.0.1:${dfxPort}`,
    fetch,
  });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    a.fetchRootKey().catch((err: any) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running",
      );
      console.error(err);
    });
  }

  return a;
}

function findCanisterId(name: string) {
  return execSync(`dfx canister id ${name}`, { encoding: "utf-8" }).trim();
}

export const swapCanisterId = Principal.fromText(
  process.env.SWAP_CANISTER_ID?.toString() ?? findCanisterId("swap"),
);

export function swap(identity?: Identity) {
  return createActor<Swap>(swapCanisterId, swapIdlFactory, {
    agent: agent(identity),
  });
}

export const tokenACanisterId = Principal.fromText(
  process.env.TOKEN_A_CANISTER_ID?.toString() ?? findCanisterId("token_a"),
);

export function tokenA(identity?: Identity) {
  return createActor<Token>(tokenACanisterId, tokenIdlFactory, {
    agent: agent(identity),
  });
}

export const tokenBCanisterId = Principal.fromText(
  process.env.TOKEN_B_CANISTER_ID?.toString() ?? findCanisterId("token_b"),
);

export function tokenB(identity?: Identity) {
  return createActor<Token>(tokenBCanisterId, tokenIdlFactory, {
    agent: agent(identity),
  });
}

export async function fundIdentity(
  token: ActorSubclass<Token>,
  to: Identity,
  amount: bigint,
) {
  const result = await token.icrc1_transfer({
    to: { owner: to.getPrincipal(), subaccount: [] },
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount,
  });
  expect(result).toHaveProperty("Ok");
}
