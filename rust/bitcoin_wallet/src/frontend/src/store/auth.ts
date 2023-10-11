import { get, writable } from 'svelte/store';
import { BackendActor, createActor } from '../lib/actor';
import { AuthClient } from '@dfinity/auth-client';
import type { JsonnableDelegationChain } from '@dfinity/identity/lib/cjs/identity/delegation';
import { route } from './router';
import { addNotification } from './notifications';
import { createIcApi, createMockApi, WalletApi } from '../lib/walletApi';

export type AuthenticatedState = {
  state: 'authenticated';
  actor: BackendActor;
  client: AuthClient;
  api: WalletApi;
  bc: BroadcastChannel;
  logout: () => Promise<void>;
};

export type AuthState =
  | {
      state: 'initializing-auth';
    }
  | {
      state: 'anonymous';
      actor: BackendActor;
      client: AuthClient;
    }
  | AuthenticatedState
  | {
      state: 'error';
      error: string;
      client: AuthClient;
    };

export const auth = writable<AuthState>({
  state: 'initializing-auth',
});

async function initAuth() {
  const client = await AuthClient.create();
  if (await client.isAuthenticated()) {
    authenticate(client);
  } else {
    auth.update(() => ({
      state: 'anonymous',
      actor: createActor(),
      client,
    }));
  }
}

initAuth();

export function login() {
  const currentAuth = get(auth);

  if (currentAuth.state === 'anonymous' || currentAuth.state === 'error') {
    currentAuth.client.login({
      maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000),
      identityProvider: process.env.INTERNET_IDENTITY_ADDRESS,
      onSuccess: () => authenticate(currentAuth.client),
      onError: (e) => {
        addNotification({
          message: 'Could not sign in with Internet Identity.',
          type: 'error',
        });
        auth.update(() => ({
          state: 'error',
          error: 'Could not sign in with Internet Identity: ' + e,
          client: currentAuth.client,
        }));
      },
    });
  }
}

async function logout() {
  const currentAuth = get(auth);

  if (currentAuth.state === 'authenticated') {
    await currentAuth.client.logout();
    currentAuth.bc.postMessage('logout');
    auth.update(() => ({
      state: 'anonymous',
      actor: createActor(),
      client: currentAuth.client,
    }));
  }
  route.navigate('');
}

export async function authenticate(client: AuthClient) {
  handleSessionTimeout();

  try {
    const actor = createActor({
      agentOptions: {
        identity: client.getIdentity(),
      },
    });

    const logoutBc = new BroadcastChannel('btc_wallet_logout');
    logoutBc.addEventListener('message', (e: MessageEvent<string>) => {
      if (e.data === 'logout') {
        logout();
      }
    });

    auth.update(() => ({
      state: 'authenticated',
      actor,
      client,
      api: process.env.USE_MOCK_API ? createMockApi(0.9) : createIcApi(actor),
      logout,
      bc: logoutBc,
    }));
  } catch (e) {
    auth.update(() => ({
      state: 'error',
      error: e.message || 'An error occurred',
      client,
    }));
  }
}

// set a timer when the II session will expire and log the user out
function handleSessionTimeout() {
  // upon login the localstorage items may not be set, wait for next tick
  setTimeout(() => {
    try {
      const delegation = JSON.parse(
        window.localStorage.getItem('ic-delegation')
      ) as JsonnableDelegationChain;

      const expirationTimeMs =
        Number.parseInt(delegation.delegations[0].delegation.expiration, 16) /
        1000000;

      setTimeout(() => {
        logout();
      }, expirationTimeMs - Date.now());
    } catch {
      console.error('Could not handle delegation expiry.');
    }
  });
}
