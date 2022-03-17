<script lang="ts">
  import Header from './Header.svelte';
  import Trash from 'svelte-icons/fa/FaTrash.svelte';
  import { addNotification, showError } from '../store/notifications';
  import { auth } from '../store/auth';
  import Spinner from './Spinner.svelte';
  import { devices, refreshDevices } from '../store/devices';

  let removing: Record<string, boolean> = {};
  let thisDevice;

  if ($auth.state === 'initialized') {
    thisDevice = $auth.crypto.deviceAlias;
  }

  async function remove(deviceAlias: string) {
    if ($auth.state !== 'initialized') {
      return;
    }

    removing[deviceAlias] = true;

    await $auth.actor.remove_device(deviceAlias).catch((e) => {
      removing[deviceAlias] = false;
      showError(e, 'Could not remove device.');
    });

    addNotification({
      type: 'success',
      message: 'Device removed successfully',
    });

    await refreshDevices($auth.actor);

    removing[deviceAlias] = false;
  }
</script>

<Header>
  <span slot="title">Connected devices </span>
</Header>

<main class="p-4">
  {#if $devices.state === 'loading' || $devices.state === 'uninitialized'}
    <div class=" flex items-center">
      <Spinner /> Loading devices...
    </div>
  {:else if $devices.state === 'error'}
    <div class="alert alert-error">Could not load devices.</div>
  {:else if $devices.state === 'loaded'}
    <ul class="border rounded-lg max-w-lg">
      {#each $devices.list as device (device[0])}
        <li class="border-b px-3 last:border-none flex items-center">
          <code class="flex-1 py-5 font-mono">{device[0]}</code>

          {#if thisDevice === device[0]}
            <span class="italic opacity-70 pr-3">this device</span>
          {:else}
            <button
              class="btn btn-sm btn-ghost  {removing[device[0]]
                ? 'loading'
                : ''}"
              on:click={() => remove(device[0])}
              disabled={removing[device[0]]}
            >
              {#if !removing[device[0]]}
                <span class="w-6 h-6 p-1"><Trash /></span>
              {:else}
                Removing...
              {/if}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</main>
