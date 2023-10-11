<script lang="ts">
  import BtcIcon from '../components/BtcIcon.svelte';
  import DfinityLogo from '../components/DfinityLogo.svelte';
  import { login } from '../store/auth';

  export let error: string | null = null;

  function doLogin() {
    login();
  }
</script>

<div class="flex flex-col min-h-screen pb-5">
  <h1 class="font-bold text-4xl md:text-7xl mt-16">
    Sample <br />Bitcoin Wallet
  </h1>

  <div class="mt-14 mb-6 md:mb-10 text-gray-500 text-lg">
    <BtcIcon class="inline-block align-top mr-3" />Your BTC assets will appear
    here
  </div>
  {#if error}
    <div class="mb-6">{error}</div>
  {/if}
  <div class="flex flex-col md:flex-row md:space-x-8 items-center">
    <button class="btn btn-black" on:click={doLogin}>
      <DfinityLogo class="inline-block mr-3" />Log in with Internet Identity
    </button>

    <a
      href="https://github.com/dfinity/examples"
      target="_blank"
      rel="noopener noreferrer"
      class="text-blue-500 underline mt-8 md:mt-0">See how it works</a
    >
  </div>
  <div class="mt-10 text-left text-sm text-gray-500 max-w-lg">
    <strong>Disclaimer:</strong> This dapp is meant for demonstration purposes only,
    and is not suitable for holding real Bitcoin.
  </div>
  <div class="flex-1" />
  <img src="/img/badge.png" alt="" class="w-72 self-center" />
</div>
