<script lang="ts">
  import BackIcon from '../components/BackIcon.svelte';
  import Drawer from '../components/Drawer.svelte';
  import SendIcon from '../components/SendIcon.svelte';
  import SpinnerIcon from '../components/SpinnerIcon.svelte';
  import { formatSats } from '../lib/formatting';
  import type { AuthenticatedState } from '../store/auth';
  import { addNotification, showError } from '../store/notifications';
  import { route } from '../store/router';

  export let auth: AuthenticatedState;

  let openConfirmationDrawer = false;
  let amount = 0;
  let address = '';
  let feePreset = '';

  function loadBalance() {
    return auth.api
      .getBalance()
      .catch((e) => showError(e, 'Could not load wallet balance.'));
  }

  let balance = loadBalance();

  function loadFees() {
    return auth.api
      .getFees()
      .then((fees) => {
        setTimeout(() => {
          // wait a tick until the options are in the DOM
          feePreset = 'std';
          fee = fees[feePreset];
        });
        return fees;
      })
      .catch((e) => {
        feePreset = 'custom';
        showError(e, 'Error loading network fees');
      });
  }

  let networkFees = loadFees();
  let fee = 0;

  let sending: Promise<void> | null = null;
  function send(address: string, amount: number, fee: number) {
    console.log({
      address,
      amount,
      fee,
    });

    sending = auth.api
      .send(address, BigInt(Math.round(amount * 10 ** 8)), BigInt(fee))
      .then(() => {
        addNotification({
          message: 'Transaction sent!',
          type: 'success',
        });
        route.navigate('');
      })
      .catch((e) =>
        addNotification({
          message: 'Error sending transaction: ' + e.message,
          type: 'error',
        })
      );
  }

  function selectFeePreset(e: Event) {
    const code = (e.target as HTMLSelectElement).value;
    if (code !== 'custom') {
      networkFees.then((fees) => {
        fee = fees[code];
      });
    }
  }
</script>

<div class="flex flex-col min-h-screen">
  <nav class="flex justify-between mt-10 md:mt-16">
    <button class="btn-icon btn-black " on:click={() => route.navigate('')}>
      <BackIcon />
    </button>
  </nav>

  <h1 class="font-bold text-4xl mt-5 md:text-7xl md:mt-8 relative mb-10">
    Send Bitcoin
  </h1>

  <form class="space-y-10 max-w-md">
    <div class="flex flex-col">
      <label for="address" class="text-lg font-medium mb-3">Address</label>
      <input
        type="text"
        id="address"
        class="h-12 border border-gray-500 rounded px-3"
        placeholder="Recipient address..."
        bind:value={address}
      />
    </div>
    <div class="flex flex-col">
      <div class="mb-3 flex justify-between">
        <label for="amount" class="text-lg font-medium ">Amount</label>
        <div class="text-xs self-center">
          {#await balance}
            <span class="animate-pulse w-36 inline-block bg-gray-300 rounded"
              >&nbsp;</span
            >
          {:then balance}
            Available:
            <button
              type="button"
              class="text-blue-500 underline  w-20"
              on:click={() => (amount = Number(balance) / 10 ** 8)}
            >
              {formatSats(balance)}
            </button>
          {:catch}
            <button
              class="text-red-500 h-7"
              on:click={() => (balance = loadBalance())}
              >Retry loading balance</button
            >
          {/await}
        </div>
      </div>
      <input
        type="number"
        step="0.00000001"
        id="amount"
        placeholder="Amount in BTC"
        class="h-12 border border-gray-500 rounded text-right px-3"
        bind:value={amount}
      />
    </div>
    <div class="flex flex-col">
      <div class="mb-3 flex justify-between">
        <label for="network-fee" class="text-lg font-medium">Network fee</label>
        <div class="text-right text-xs self-center">
          {#await networkFees}
            <span class="animate-pulse w-20 inline-block bg-gray-300 rounded"
              >&nbsp;</span
            >
          {:then}
            {fee} sat/b
          {:catch}
            <button
              class="text-red-500 h-7"
              on:click={() => (networkFees = loadFees())}
              >Retry loading fees</button
            >
          {/await}
        </div>
      </div>
      <select
        id="network-fee"
        class="h-12 border border-gray-500 rounded text-left px-3"
        bind:value={feePreset}
        on:change={selectFeePreset}
      >
        {#await networkFees}
          <option value="" disabled>Loading network fees...</option>
          <!-- <option value="low" disabled>Low</option> -->
          <!-- <option value="std" disabled>Standard</option> -->
          <!-- <option value="high" disabled>High</option> -->
        {:then}
          <option value="low">Low</option>
          <option value="std">Standard</option>
          <option value="high">High</option>
          <!-- {:catch} -->
        {/await}
        <option value="custom">Custom</option>
      </select>
    </div>
    {#if feePreset === 'custom'}
      <div class="flex flex-col">
        <label for="customFee" class="text-lg font-medium mb-3"
          >Custom Fee, sat/b</label
        >
        <input
          type="number"
          step="1"
          id="customFee"
          class="h-12 border border-gray-500 rounded text-right px-3 max-w-[250px]"
          bind:value={fee}
        />
      </div>
    {/if}
    <div class="pt-4">
      {#await sending}
        <button class="btn btn-blue w-full space-x-1" type="button" disabled>
          <SpinnerIcon />
          <span>Sending...</span></button
        >
      {:then}
        <button
          class="btn btn-blue w-full space-x-1"
          type="button"
          disabled={address.trim().length === 0 || !amount}
          on:click={() => send(address, amount, fee)}
          ><SendIcon /><span>Send</span></button
        >
      {:catch}
        <p class="text-xl text-red-500 mb-4">Error sending transaction.</p>
        <button
          class="btn btn-blue w-full space-x-1"
          type="button"
          disabled={address.trim().length === 0 || !amount}
          on:click={() => send(address, amount, fee)}
          ><SendIcon /><span>Try again</span></button
        >
      {/await}
    </div>
  </form>
  <div class="flex-1 hidden md:block" />
  <img src="/img/badge.png" alt="" class="w-72 self-center pb-5 mt-8" />
</div>

<Drawer
  open={openConfirmationDrawer}
  on:close={() => (openConfirmationDrawer = false)}
>
  <div class="flex flex-col items-center">
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

    <h2 class="text-lg font-medium">Derive new address?</h2>
    <span class="text-lg text-gray-500 mb-2"
      >This will update the displayed address and the QR code
    </span>
    <button
      class="btn btn-black w-full"
      on:click={() => (openConfirmationDrawer = false)}
      >Derive new address</button
    >
  </div>
</Drawer>
