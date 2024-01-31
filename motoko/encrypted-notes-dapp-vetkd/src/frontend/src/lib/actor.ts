import {
  Actor,
  ActorConfig,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
} from '@dfinity/agent';
import { ENCRYPTED_NOTES_CANISTER_ID, _SERVICE } from './backend';
import { idlFactory } from './backend';

export type BackendActor = ActorSubclass<_SERVICE>;

export function createActor(options?: {
  agentOptions?: HttpAgentOptions;
  actorOptions?: ActorConfig;
}): BackendActor {
  const hostOptions = {
    host:
      process.env.DFX_NETWORK === 'ic'
        ? `https://${ENCRYPTED_NOTES_CANISTER_ID}.ic0.app`
        : 'http://localhost:8000',
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
  if (process.env.NODE_ENV !== 'production') {
    console.log(`Dev environment - fetching root key...`);

    agent.fetchRootKey().catch((err) => {
      console.warn(
        'Unable to fetch root key. Check to ensure that your local replica is running'
      );
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId: ENCRYPTED_NOTES_CANISTER_ID,
    ...options?.actorOptions,
  });
}
