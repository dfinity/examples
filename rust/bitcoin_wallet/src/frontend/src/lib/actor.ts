import {
  Actor,
  ActorConfig,
  ActorSubclass,
  HttpAgent,
  HttpAgentOptions,
} from '@dfinity/agent';
import { addNotification } from '../store/notifications';
import { BACKEND_CANISTER_ID, _SERVICE } from './backend';
import { idlFactory } from './backend';

export type BackendActor = ActorSubclass<_SERVICE>;

export function createActor(options?: {
  agentOptions?: HttpAgentOptions;
  actorOptions?: ActorConfig;
}): BackendActor {
  const hostOptions = {
    host:
      process.env.DFX_NETWORK === 'ic'
        ? `https://${BACKEND_CANISTER_ID}.ic0.app`
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
  if (process.env.DFX_NETWORK !== 'ic') {
    console.log(`Dev environment - fetching root key...`);

    agent.fetchRootKey().catch((err) => {
      console.warn(
        'Unable to fetch root key. Check to ensure that your local replica is running'
      );
      addNotification({ message: 'Could not fetch root ket.', type: 'error' });
      // console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId: BACKEND_CANISTER_ID,
    ...options?.actorOptions,
  });
}
