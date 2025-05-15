<script>
  import { AuthClient } from "@dfinity/auth-client";
  import { onMount } from "svelte";
  import { auth, createActor } from "../store/auth";
  import BalanceInfo from '../components/BalanceInfo.svelte';

  /** @type {AuthClient} */
  let client;

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
  }
</script>

<div class="container">
  {#if $auth.loggedIn}
    <div class="auth-btn-container">
      <button on:click={logout}>Log out</button>
    </div>
    <div>
      <BalanceInfo />
    </div>
  {:else}
    <button on:click={login}>Authenticate in with Internet Identity</button>
  {/if}
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
</style>
