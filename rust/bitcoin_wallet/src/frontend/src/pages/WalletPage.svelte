<script lang="ts">
  import BtcIcon from '../components/BtcIcon.svelte';
  import Drawer from '../components/Drawer.svelte';
  import MenuIcon from '../components/MenuIcon.svelte';
  import ReceiveIcon from '../components/ReceiveIcon.svelte';
  import SendIcon from '../components/SendIcon.svelte';
  import TextSpinner from '../components/TextSpinner.svelte';
  import { formatSats } from '../lib/formatting';
  import type { AuthenticatedState } from '../store/auth';
  import { showError } from '../store/notifications';
  import { route } from '../store/router';

  let userInfoShown = false;

  export let auth: AuthenticatedState;

  function loadBalance() {
    return auth.api
      .getBalance()
      .catch((e) => showError(e, 'Could not load wallet balance.'));
  }

  let balance = loadBalance();

  function openDrawer() {
    userInfoShown = true;
  }
  function closeDrawer() {
    userInfoShown = false;
  }

  function logout() {
    auth.logout();
    route.navigate('');
  }
</script>

<div class="flex flex-col min-h-screen">
  <h1 class="font-bold text-4xl mt-10 md:text-7xl md:mt-16 relative">
    Sample <br />Bitcoin Wallet

    <button
      class="btn-icon btn-gray absolute right-0 top-1 md:top-4"
      on:click={openDrawer}
    >
      <MenuIcon />
    </button>
  </h1>
  <div class="mt-14 mb-20 ">
    <BtcIcon class="inline-block align-top mr-3" /><span
      class="text-gray-500 text-lg">BTC</span
    >
    <div class="text-5xl md:text-6xl mt-2">
      {#await balance}
        <span class="">Loading balance<TextSpinner /></span>
      {:then b}
        {formatSats(b)}
      {:catch}
        <button
          class="text-red-500 text-xl md:text-2xl "
          on:click={() => (balance = loadBalance())}
          >Could not load balance. Try again.
        </button>
      {/await}
    </div>
  </div>
  <div class="flex-1 md:hidden" />
  <div class="flex space-x-5">
    <button
      class="w-56 btn btn-blue space-x-1"
      on:click={() => route.navigate('receive')}
      ><ReceiveIcon /><span>Receive</span></button
    >
    <button
      class="w-56 btn btn-blue space-x-1"
      on:click={() => route.navigate('send')}
    >
      <SendIcon />
      <span>Send</span></button
    >
  </div>
  <div class="flex-1 hidden md:block" />
  <img src="/img/badge.png" alt="" class="w-72 self-center pb-5 mt-8" />
</div>
<Drawer open={userInfoShown} on:close={closeDrawer}>
  <div class="flex flex-col items-center">
    <span class="text-lg text-gray-500 mb-2">Your Principal</span>
    <span class="text-xl mb-16 text-center"
      >{auth.client.getIdentity().getPrincipal().toString()}</span
    >
    <button class="btn btn-black w-full" on:click={logout}>Log out</button>
  </div>
</Drawer>
