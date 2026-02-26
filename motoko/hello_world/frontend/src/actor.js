import { HttpAgent } from "@icp-sdk/core/agent";
import { safeGetCanisterEnv } from "@icp-sdk/core/agent/canister-env";
import { createActor } from "./bindings/backend";

// --- Environment resolution ---

const canisterEnv = safeGetCanisterEnv();

// Resolve canister ID: cookie (icp-cli) → env var (dfx)
const canisterId = (() => {
  if (canisterEnv) {
    const cookieId = canisterEnv["PUBLIC_CANISTER_ID:backend"];
    if (cookieId) return cookieId;
  }
  if (process.env.CANISTER_ID_BACKEND) return process.env.CANISTER_ID_BACKEND;
  throw new Error(
    "Canister ID for 'backend' not found. Run 'icp deploy' or 'dfx deploy' first."
  );
})();

// --- Agent and actor creation ---

function createAgent() {
  const options = { host: window.location.origin };

  // icp-cli: root key is provided via the ic_env cookie (already decoded to Uint8Array)
  if (canisterEnv?.IC_ROOT_KEY) {
    options.rootKey = canisterEnv.IC_ROOT_KEY;
  }

  const agent = HttpAgent.createSync(options);

  // dfx local: fetch root key from the replica (not needed on mainnet or with icp-cli)
  if (!canisterEnv && process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  return agent;
}

export const backend = createActor(canisterId, { agent: createAgent() });
