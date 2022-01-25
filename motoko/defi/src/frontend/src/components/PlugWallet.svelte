<script>
    import { onMount } from 'svelte';
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';

    let plugWalletInstalled = window.ic ? true : false;
    let plugWalletConnected;
    let plugActor;

    const DEX_CANISTER_ID = process.env.DEFI_DAPP_CANISTER_ID;
    const LEDGER_CANISTER_ID = process.env.LEDGER_CANISTER_ID;
    const AKITADIP20_CANISTER_ID = process.env.AKITADIP20_CANISTER_ID;
    const GOLDENDIP20_CANISTER_ID = process.env.GOLDENDIP20_CANISTER_ID;
    const whiteList = [DEX_CANISTER_ID, LEDGER_CANISTER_ID, AKITADIP20_CANISTER_ID, GOLDENDIP20_CANISTER_ID];

    onMount(async () => {
        if(plugWalletInstalled) {
            if(await window.ic.plug.isConnected) {
                plugWalletConnected = true; 
                // create plug actor        
                await window.ic.plug.createAgent({whiteList});
                window.ic.plug.agent.fetchRootKey();
                plugActor = await window.ic.plug.createActor({
                    canisterId: DEX_CANISTER_ID,
                    interfaceFactory: idlFactory
                });
                console.log(plugActor)
                const test = await plugActor.list_order();
                console.log(test)

            }
            else {
                let requestConnection = await window.ic.plug.requestConnect();
                if(requestConnection) {
                    plugWalletConnected = true;
                }
                else {
                    plugWalletConnected = false;
                }
            }
        }
	});

    async function connectToPlugWallet() {
        if(window.ic.plug) {
            try {       
                let connected = await window.ic.plug.requestConnect({whiteList});
                if(connected) {
                   plugWalletConnected = true;
                   // create plug actor
                   plugActor = window.ic.plug.createActor({
                       canisterId: DEX_CANISTER_ID,
                       interfaceFactory: idlFactory,
                   });
                   const test = await plugActor.list_order();
                }
           }
           catch {
               console.log('Did not connect Plug Wallet')
           }
        }
    }
</script>

<div>
    {#if plugWalletInstalled}
        <div>
            {#if plugWalletConnected}
                <button on:click={connectToPlugWallet} >Connect to Plug Wallet</button>
                {:else}
                <p>You plug wallet is connected!</p>
            {/if}
        </div>
    {:else}
        <div>
            <p>You do not appear to have the Plug Wallet Chrome extension installed</p>
            <p>To Connect to your Plug Wallet, install the Chrome Extension</p>
            <p>
                <a  target='_blank'
                rel='noopener noreferrer'
                href="https://chrome.google.com/webstore/detail/plug/cfbfdhimifdmdehjmkdobpcjfefblkjm">
                Download Here
                </a>
            </p>
        </div>  
    {/if}
</div>
