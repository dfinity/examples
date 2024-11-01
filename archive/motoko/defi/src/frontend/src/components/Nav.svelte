<script>
    import { AuthClient } from "@dfinity/auth-client";
    import { onMount } from "svelte";
    import { auth, createActor, plugWallet, whitelist, host,
        DEX_CANISTER_ID, AKITA_CANISTER_ID, GOLDENDIP20_CANISTER_ID, LEDGER_CANISTER_ID } from "../store/auth";
    import { idlFactory as akitaIDL } from "../../declarations/AkitaDIP20/AkitaDIP20.did.js";
    import { idlFactory as goldenIDL } from "../../declarations/GoldenDIP20/GoldenDIP20.did.js";
    import { idlFactory as backendIDL} from "../../declarations/defi_dapp/defi_dapp.did.js";
    import { idlFactory as ledgerIDL} from "../../declarations/ledger/ledger.did.js";
    /** @type {AuthClient} */
    let client;
    // Plug wallet connection request
    onMount(async () => {
        // Internet Identity:q
        client = await AuthClient.create();
        const id = client.getIdentity();
        if (await client.isAuthenticated()) {
            handleAuth();
        }
        // TODO: Support Plug wallet
        // if(!await window.ic.plug.isConnected()){
        //     console.log("connect to plug");
        //     await requestPlugConnection();
        //     console.log("finished connect to plug");
        // }
	});

    // TODO: Support Plug wallet
    async function requestPlugConnection() {
        try {
            // Request to connect plug wallet. This request permission from user to interact with backend
            // Local deployment whitelist will not get added correctly since Plug check with canisters deployed on IC
            // https://github.com/Psychedelic/plug/blob/3ce6b32e9d081b90f6b5ebd2926236b8d38ecfd2/source/Background/Controller.js#L180
            console.log(host)
            await window.ic.plug.requestConnect({whitelist: whitelist, host: host});
            
            if(process.env.DFX_NETWORK === 'local') {
                
                await window.ic.plug.createAgent({whitelist:whitelist, host: host})
                await window.ic.plug.agent.fetchRootKey();
            }
            const principal = await window.ic.plug.agent.getPrincipal();
            const plugActor = await window.ic.plug.createActor({
                canisterId: DEX_CANISTER_ID,
                interfaceFactory: backendIDL
            });
            const plugAkitaActor = await window.ic.plug.createActor({
                canisterId: AKITA_CANISTER_ID,
                interfaceFactory: akitaIDL
            });
            const plugGoldenActor = await window.ic.plug.createActor({
                canisterId: GOLDENDIP20_CANISTER_ID,
                interfaceFactory: goldenIDL
            });
            const plugLedgerActor = await window.ic.plug.createActor({
                canisterId: LEDGER_CANISTER_ID,
                interfaceFactory: ledgerIDL
            });
            plugWallet.set({...$plugWallet, principal, plugActor, plugAkitaActor, plugGoldenActor, plugLedgerActor, isConnected: true});     
            console.log("akita name:" , await plugAkitaActor.name());
            console.log("golden name:" , await plugGoldenActor.name());       
            console.log("defi balances:" , await plugActor.getBalances());       
        } catch (e) {
            console.log(e);
        }
    };

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
      <!--Due to lack of support for local testing Plug wallet, Plug wallet auth button
      will be commented out. This dapp has the foundation for plug integration within the code-->
      <!--
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
      -->
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
