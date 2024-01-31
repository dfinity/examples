import { get, writable } from 'svelte/store';
import type { BackendActor } from '../lib/actor';
import { auth, logout } from './auth';
import { showError } from './notifications';

export type DeviceEntry = [alias: string, publicKey: string];

export const devices = writable<
  | {
      state: 'uninitialized';
    }
  | {
      state: 'loading';
    }
  | {
      state: 'loaded';
      list: DeviceEntry[];
    }
  | {
      state: 'error';
    }
>({ state: 'uninitialized' });

let devicePollerHandle: ReturnType<typeof setInterval> | null;

export async function refreshDevices(
  actor: BackendActor
): Promise<DeviceEntry[]> {
  const list = await actor.get_devices();
  devices.update(($devices) => {
    if ($devices.state === 'uninitialized') return $devices;
    return {
      state: 'loaded',
      list,
    };
  });
  return list;
}

auth.subscribe(async ($auth) => {
  if ($auth.state === 'initialized') {
    if (devicePollerHandle !== null) {
      clearInterval(devicePollerHandle);
      devicePollerHandle = null;
    }

    devices.set({
      state: 'loading',
    });
    try {
      await refreshDevices($auth.actor);
      devicePollerHandle = setInterval(async () => {
        const $auth = get(auth);

        if ($auth.state !== 'initialized') return;

        const devices = await refreshDevices($auth.actor).catch((e) =>
          showError(e, 'Could not poll devices.')
        );

        if (!devices.find(([alias, _]) => alias === $auth.crypto.deviceAlias)) {
          console.log('Device removed.');
          await $auth.crypto.clearDevice();
          await logout();
        }
      }, 1000);
    } catch {
      devices.set({
        state: 'error',
      });
    }
  } else if ($auth.state === 'anonymous' && devicePollerHandle) {
    clearInterval(devicePollerHandle);
    devicePollerHandle = null;
    devices.set({
      state: 'uninitialized',
    });
  }
});
