<script>
    import { plugWallet } from '../store/auth';
    import { AuthClient } from "@dfinity/auth-client";
    import { onMount } from "svelte";
    import { auth, createActor } from "../store/auth";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { Principal } from '@dfinity/principal';

    
    const DEX_CANISTER_ID = process.env.DEFI_DAPP_CANISTER_ID;
    const whiteList = [DEX_CANISTER_ID];

    /** @type {AuthClient} */
    let client;

// Plug wallet connection request
    onMount(async () => {
        // Internet Identity
        client = await AuthClient.create();
        if (await client.isAuthenticated()) {
            handleAuth();
        }

        // Plug wallet 
        plugWallet.set({
            ...$plugWallet,
            isConnected:  await window.ic.plug.isConnected()
        });

        if($plugWallet.isConnected) {
            // create plug actor   
            const principal = await window.ic.plug.getPrincipal();
            await window.ic.plug.createAgent({whiteList});
            window.ic.plug.agent.fetchRootKey();
            plugWallet.set({...$plugWallet, principal, plugActor: await window.ic.plug.createActor({
                canisterId: DEX_CANISTER_ID,
                interfaceFactory: idlFactory
            })});
        }
	});

    async function requestPlugConnection() {
        try {
            const publicAddress = await window.ic.plug.requestConnect(whiteList, "http://localhost:5000");
            console.log(`The connected user's public key is:`, publicAddress);
            const principal = await window.ic.plug.getPrincipal();
            plugWallet.set({...$plugWallet, publicAddress, principal, isConnected: true})
        } catch (e) {
            console.log(e);
        }
    };

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

<div id="nav-container">
    <a
      href="https://dfinity.org"
      target="_blank"
      rel="noopener noreferrer"
      class="logo"
    >
      <img src="images/dfinity.svg" alt="DFINITY logo" />
    </a>
    <ul>
      <li>
            {#if $auth.loggedIn}
                <button on:click={logout}>Log out</button>
            {:else}
                <button on:click={login}>Login</button>
            {/if}
      </li>
      <li>
            {#if !$plugWallet.isConnected} 
                <button on:click={requestPlugConnection}>Connect To Plug</button>
            {:else}
                <button>{$plugWallet.principal}</button>
            {/if}
      </li>
    </ul>
</div>

<style>
    #nav-container {
        display: inline-flex;
        width: 100%;
    }

    li {
      display: inline-flex;
      padding: 10px
    }
    ul {
      margin-left: auto;
      margin-top: -15px;
      padding: 0;
    }
    img {
      height: 22px;
    }
    .logo {
      display: inline-block;
    }
</style>