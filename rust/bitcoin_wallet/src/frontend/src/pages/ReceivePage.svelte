<script lang="ts">
  import BackIcon from '../components/BackIcon.svelte';
  import CopyIcon from '../components/CopyIcon.svelte';
  // import DeriveIcon from '../components/DeriveIcon.svelte';
  import DotsSpinner from '../components/DotsSpinner.svelte';
  import Drawer from '../components/Drawer.svelte';
  import SpinnerIcon from '../components/SpinnerIcon.svelte';
  import TextSpinner from '../components/TextSpinner.svelte';
  import QrCode from '../lib/qrcode';
  import type { AuthenticatedState } from '../store/auth';
  import { route } from '../store/router';

  let openConfirmationDrawer = false;

  export let auth: AuthenticatedState;

  let address = auth.api.getAddress();

  function deriveNewAddress() {
    return auth.api.deriveNewAddress().then(() => {
      openConfirmationDrawer = false;
      address = auth.api.getAddress();
    });
  }

  let derivingNewAddress: Promise<void> | null;

  function qrcode(node: HTMLCanvasElement, address: string) {
    new (QrCode as any)({
      element: node,
      value: address,
      size: 512,
    });
  }

  function copyToClipboard(address) {
    navigator.clipboard.writeText(address);
  }
</script>

<div class="flex flex-col md:block min-h-screen pb-12">
  <nav class="flex justify-between mt-10 md:mt-16">
    <button class="btn-icon btn-black " on:click={() => route.navigate('')}>
      <BackIcon />
    </button>
    <!-- <button
      class="btn-icon btn-gray "
      on:click={() => (openConfirmationDrawer = true)}
    >
      <DeriveIcon />
    </button> -->
  </nav>

  <h1 class="font-bold text-4xl mt-5 md:text-7xl md:mt-8 relative">
    Receive Bitcoin
  </h1>

  <div class="text-center mt-12 flex justify-center">
    {#await address}
      <div
        class="w-48 h-48 border-black border-solid border bg-white flex flex-col justify-center items-center gap-4"
      >
        <DotsSpinner />
        Loading QR code
      </div>
    {:then address}
      <canvas class="w-48 h-48" use:qrcode={address} />
    {:catch}
      <div
        class="w-48 h-48 p-4 border-black border-solid border bg-white flex flex-col justify-center items-center gap-4"
      >
        Could not load address.
      </div>
    {/await}
  </div>

  <div class="mt-14 text-lg">Your BTC address</div>
  <div class="flex items-center mt-3 space-x-6">
    {#await address}
      <div class="flex-1">
        <span class="text-xl font-medium">
          Loading address<TextSpinner />
        </span>
      </div>
    {:then address}
      <div class="flex-1">
        <span class="text-xl break-all font-medium">
          {address}
        </span>
      </div>
      <button
        class="btn-icon btn-black"
        on:click={() => copyToClipboard(address)}
      >
        <CopyIcon />
      </button>
    {:catch}
      <button
        class="text-red-500 text-xl"
        on:click={() => (address = auth.api.getAddress())}
        >Could not load address. Try again.</button
      >
    {/await}
  </div>
</div>

<Drawer
  open={openConfirmationDrawer}
  on:close={() => (openConfirmationDrawer = false)}
>
  <div class="flex flex-col items-center text-center">
    <svg
      width="72"
      height="72"
      viewBox="0 0 72 72"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      class="mb-2"
    >
      <path
        d="M10 58L35.5 15L61 58H10ZM35.5 53.1849C36.4653 53.1849 37.2865 52.849 37.9635 52.1771C38.653 51.4928 38.9978 50.6716 38.9978 49.7135C38.9978 48.7555 38.653 47.9405 37.9635 47.2687C37.2865 46.5843 36.4653 46.2422 35.5 46.2422C34.5347 46.2422 33.7072 46.5843 33.0177 47.2687C32.3407 47.9405 32.0022 48.7555 32.0022 49.7135C32.0022 50.6716 32.3407 51.4928 33.0177 52.1771C33.7072 52.849 34.5347 53.1849 35.5 53.1849ZM33.2057 42.0243H37.7942L38.1327 29.3333H32.8673L33.2057 42.0243Z"
        fill="black"
      />
    </svg>

    <h2 class="text-lg font-medium mb-3">Derive new address?</h2>
    <span class="text-lg text-gray-500 mb-12"
      >This will update the displayed address and the QR code
    </span>
    {#await derivingNewAddress}
      <button class="btn btn-black w-full" disabled>
        <SpinnerIcon />
        Deriving new address...</button
      >
    {:then _}
      <button
        class="btn btn-black w-full"
        on:click={() => (derivingNewAddress = deriveNewAddress())}
        >Derive new address</button
      >
    {:catch}
      <p class="text-xl text-red-500 mb-4">Error deriving new address.</p>
      <button
        class="btn btn-black w-full"
        on:click={() => (derivingNewAddress = deriveNewAddress())}
        >Try again</button
      >
    {/await}
  </div>
</Drawer>
