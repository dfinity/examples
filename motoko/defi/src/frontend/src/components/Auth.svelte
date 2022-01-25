<script>
  import { AuthClient } from "@dfinity/auth-client";
  import { onMount } from "svelte";
  import { auth, createActor } from "../store/auth";
  import PlugWallet from '../components/PlugWallet.svelte';
  import BalanceInfo from '../components/BalanceInfo.svelte';

  /** @type {AuthClient} */
  let client;

  let whoami = $auth.actor.whoami();

  onMount(async () => {
    client = await AuthClient.create();
    if (await client.isAuthenticated()) {
      handleAuth();
    }
  });

  function handleAuth() {
    auth.update(() => ({
      loggedIn: true,
      actor: createActor({
        agentOptions: {
          identity: client.getIdentity(),
        },
      }),
    }));

    whoami = $auth.actor.whoami();
  }

  function login() {
    client.login({
      identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app/#authorize"
          : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
      onSuccess: handleAuth,
    });
  }

  async function logout() {
    await client.logout();
    auth.update(() => ({
      loggedIn: false,
      actor: createActor(),
    }));

    whoami = $auth.actor.whoami();
  }
</script>

<div class="container">
  {#if $auth.loggedIn}
    <div class="auth-btn-container">
      <PlugWallet />
      <button on:click={logout}>Log out</button>
    </div>
    <div>
      <BalanceInfo />
    </div>
  {:else}
    <button on:click={login}>Authenticate in with Internet Identity</button>
  {/if}

  <div class="principal-info">
    {#await whoami}
      Querying caller identity...
    {:then principal}
      Your principal ID is
      <code>{principal}</code>

      {#if principal.isAnonymous()}
        (anonymous)
      {/if}
    {/await}
  </div>
</div>

<style>
  .container {
    margin: 64px 0;
  }

  .auth-btn-container {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .principal-info {
    margin-top: 32px;
  }
</style>
