<script lang="ts">
  import BackIcon from '../components/BackIcon.svelte';
  import DotsSpinner from '../components/DotsSpinner.svelte';
  import Drawer from '../components/Drawer.svelte';
  import SendIcon from '../components/SendIcon.svelte';
  import SpinnerIcon from '../components/SpinnerIcon.svelte';
  import { enumIs } from '../lib/enums';
  import { formatSats } from '../lib/formatting';
  import type { AuthenticatedState } from '../store/auth';
  import { addNotification, showError } from '../store/notifications';
  import { route } from '../store/router';

  export let auth: AuthenticatedState;

  let openSendConfirmationDrawer = false;
  let amount: number | null = null;
  let address = '';
  let feePreset = '';
  let maxAmount: number | null = null;

  function loadBalance() {
    return auth.api
      .getBalance()
      .then((b) => {
        maxAmount = Number(b) / 10 ** 8;
        return b;
      })
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
  let fee: number = 0;

  let sending: Promise<void> | null = null;
  function send(address: string, amount: number, fee: number) {
    console.log({
      address,
      amount,
      fee,
    });

    sending = auth.api
      .send(address, BigInt(Math.round(amount * 10 ** 8)), BigInt(fee * 1000))
      .then((result) => {
        console.log(result);
        if (enumIs(result, 'Ok')) {
          addNotification(
            {
              message: 'Transaction sent with id ' + result.Ok.id,
              type: 'success',
            },
            0
          );
          route.navigate('');
        } else {
          if (enumIs(result.Err, 'InsufficientBalance')) {
            throw new Error('Insufficient balance');
          } else if (enumIs(result.Err, 'InvalidPercentile')) {
            throw new Error('Invalid Percentile');
          } else if (enumIs(result.Err, 'MalformedDestinationAddress')) {
            throw new Error('Malformed Destination Address');
          } else if (enumIs(result.Err, 'ManagementCanisterReject')) {
            throw new Error('Management Canister Reject');
          } else if (enumIs(result.Err, 'MinConfirmationsTooHigh')) {
            throw new Error('Min Confirmations Too High');
          } else if (enumIs(result.Err, 'UnsupportedSourceAddressType')) {
            throw new Error('Unsupported Source Address Type');
          }
        }
      })
      .catch((e) =>
        addNotification(
          {
            message: 'Error sending transaction: ' + e.message,
            type: 'error',
          },
          0
        )
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
            <DotsSpinner class="relative -left-7" />
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
        min="0.00001"
        max={maxAmount}
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
            <DotsSpinner class="relative -left-7" />
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
          disabled={address.trim().length === 0 ||
            amount < 0.00001 ||
            amount > maxAmount ||
            fee == 0}
          on:click={() => (openSendConfirmationDrawer = true)}
          ><SendIcon /><span>Confirm & Send</span></button
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
  open={openSendConfirmationDrawer}
  on:close={() => (openSendConfirmationDrawer = false)}
>
  <div class="flex flex-col items-center">
    <SendIcon class="text-black w-16 h-16" />

    <h2 class="text-lg font-medium mb-3">Confirm transaction</h2>
    <span class="text-lg text-gray-500 mb-12 max-w-lg text-center mx-auto"
      >You're about to send <strong class="text-black">{amount} BTC</strong>
      to <strong class="text-black block text-center">{address}</strong>
      with a transaction fee of
      <strong class="text-black">{fee} sats/b</strong>.
    </span>
    <div class="flex gap-5 w-full">
      <button
        class="btn btn-blue flex-1"
        on:click={() => {
          send(address, amount, fee);
          openSendConfirmationDrawer = false;
        }}>Send</button
      >
      <button
        class="btn btn-gray flex-1"
        on:click={() => (openSendConfirmationDrawer = false)}>Cancel</button
      >
    </div>
  </div>
</Drawer>
