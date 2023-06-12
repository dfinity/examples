import { Actor, HttpAgent } from "@dfinity/agent";

// Imports and re-exports candid interface
import { idlFactory } from './nftwallet.did.js';
export { idlFactory } from './nftwallet.did.js';
// CANISTER_ID is replaced by webpack based on node environment
export const canisterId = process.env.NFTWALLET_CANISTER_ID;

/**
 * 
 * @param {string | import("@dfinity/principal").Principal} canisterId Canister ID of Agent
 * @param {{agentOptions?: import("@dfinity/agent").HttpAgentOptions; actorOptions?: import("@dfinity/agent").ActorConfig}} [options]
 * @return {import("@dfinity/agent").ActorSubclass<import("./nftwallet.did.js")._SERVICE>}
 */
 export const createActor = (canisterId, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });
  
  // Fetch root key for certificate validation during development
  if(process.env.NODE_ENV !== "production") {
    console.log("Fetching root key");
    agent.fetchRootKey().catch(err=>{
      console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
      console.error(err);
    });
  } else {
    console.log("Not fetching root key");
    console.log(process.env.NODE_ENV);
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options?.actorOptions,
  });
};
  
/**
 * A ready-to-use agent for the nftwallet canister
 * @type {import("@dfinity/agent").ActorSubclass<import("./nftwallet.did.js")._SERVICE>}
 */
//  export const nftwallet = createActor(canisterId);
