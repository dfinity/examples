<script>
    import { plugWallet } from '../store/auth';
    import { AuthClient } from "@dfinity/auth-client";
    import { onMount } from "svelte";
    import { auth, createActor } from "../store/auth";
    import { idlFactory } from '../../declarations/defi_dapp/defi_dapp.did.js';

    
    const DEX_CANISTER_ID = process.env.DEFI_DAPP_CANISTER_ID;
    const whiteList = [DEX_CANISTER_ID];

    /** @type {AuthClient} */
    let client;

    // Plug wallet connection request
    onMount(async () => {
        // Internet Identity
        client = await AuthClient.create();
        const id = client.getIdentity();
        if (await client.isAuthenticated()) {
            handleAuth();
        }

        if($plugWallet.isConnected)
            setPlugWalletInfo();

	});

    async function requestPlugConnection() {
        try {
            const host = process.env.DFX_NETWORK === "ic"
                ? `https://${process.env.DEFI_DAPP_CANISTER_ID}.ic0.app`
                : "http://localhost:8000";
            await window.ic.plug.requestConnect(whiteList, host);
            setPlugWalletInfo();
        } catch (e) {
            console.log(e);
        }
    };

    async function setPlugWalletInfo() {
        // create plug actor   
        const principal = await window.ic.plug.getPrincipal();
        const plugOptions = {
            whiteList,
            host: process.env.DFX_NETWORK !== "local"
                ? `https://${process.env.DEFI_DAPP_CANISTER_ID}.ic0.app`
                : "http://localhost:8000",
        }
        await window.ic.plug.createAgent(plugOptions);
        if(process.env.DFX_NETWORK === 'local') {
            window.ic.plug.agent.fetchRootKey();
        }
        const plugActor = await window.ic.plug.createActor({
            canisterId: DEX_CANISTER_ID,
            interfaceFactory: idlFactory
        });
        plugWallet.set({...$plugWallet, principal, plugActor, isConnected: true});       
    }

    function handleAuth() {
        console.log('in handle auth');
        console.log(client.getIdentity())
        // Update Auth Store
        auth.update(() => ({
          loggedIn: true,
          principal: client.getIdentity().getPrincipal(),
          actor: createActor({
            agentOptions: {
              identity: client.getIdentity(),
            },
          }),
        }));
        // Create Canister Actors with II

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
          principal: '',
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
                <button class="top-round-rainbow" on:click={requestPlugConnection}>
                    <span>
                        <img class="plug-logo" src="images/plug_logo.png" alt="Plug logo" />
                    </span>
                    Plug
                </button>
            {:else}
                <button class="top-round-rainbow">
                    <span>
                        <img class="plug-logo" src="images/plug_logo.png" alt="Plug logo" />
                    </span>
                    {$plugWallet.principal}
                </button>
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

    .plug-logo {
        height: 16px;
    }

    .top-round-rainbow {
        background-image: repeating-linear-gradient(to right,
            #FFE701,#FC9770,#FB72A5,#C172DA);
        background-size: 100% 3px;
        background-repeat:repeat;
    }
</style>
